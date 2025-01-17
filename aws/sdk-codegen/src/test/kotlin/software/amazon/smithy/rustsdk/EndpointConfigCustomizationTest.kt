/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rustsdk

import io.kotest.matchers.shouldBe
import org.junit.jupiter.api.Test
import software.amazon.smithy.aws.traits.ServiceTrait
import software.amazon.smithy.model.shapes.ServiceShape
import software.amazon.smithy.rust.codegen.rustlang.CargoDependency
import software.amazon.smithy.rust.codegen.testutil.asSmithyModel
import software.amazon.smithy.rust.codegen.testutil.compileAndTest
import software.amazon.smithy.rust.codegen.testutil.stubConfigProject
import software.amazon.smithy.rust.codegen.testutil.unitTest
import software.amazon.smithy.rust.codegen.testutil.validateConfigCustomizations
import software.amazon.smithy.rust.codegen.util.expectTrait
import software.amazon.smithy.rust.codegen.util.lookup

internal class EndpointConfigCustomizationTest {

    private val model = """
    namespace test
    @aws.api#service(sdkId: "Test", endpointPrefix: "differentprefix")
    service TestService {
        version: "123"
    }

    @aws.api#service(sdkId: "Test")
    service NoEndpointPrefix {
        version: "123"
    }
    """.asSmithyModel()

    @Test
    fun `generates valid code`() {
        validateConfigCustomizations(EndpointConfigCustomization(AwsTestRuntimeConfig, model.lookup("test#TestService")))
    }

    @Test
    fun `generates valid code when no endpoint prefix is provided`() {
        val serviceShape = model.lookup<ServiceShape>("test#NoEndpointPrefix")
        validateConfigCustomizations(EndpointConfigCustomization(AwsTestRuntimeConfig, serviceShape))
        serviceShape.expectTrait<ServiceTrait>().endpointPrefix shouldBe "noendpointprefix"
    }

    @Test
    fun `write an endpoint into the config`() {
        val project = stubConfigProject(EndpointConfigCustomization(AwsTestRuntimeConfig, model.lookup("test#TestService")))
        project.lib {
            it.addDependency(awsTypes(AwsTestRuntimeConfig))
            it.addDependency(CargoDependency.Http)
            it.unitTest(
                """
                use aws_types::region::Region;
                use http::Uri;
                let conf = crate::config::Config::builder().build();
                let endpoint = conf.endpoint_resolver
                    .endpoint(&Region::new("us-east-1")).expect("default resolver produces a valid endpoint");
                let mut uri = Uri::from_static("/?k=v");
                endpoint.set_endpoint(&mut uri, None);
                assert_eq!(uri, Uri::from_static("https://differentprefix.us-east-1.amazonaws.com/?k=v"));
            """
            )
        }
        project.compileAndTest()
    }
}
