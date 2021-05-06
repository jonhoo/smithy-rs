$version: "1.0"
namespace aws.protocoltests.restxml

use aws.protocols#restXml
use aws.api#service
use smithy.test#httpResponseTests


/// A REST XML service that sends XML requests and responses.
@service(sdkId: "Rest Json Protocol")
@restXml
service RestXmlExtras {
    version: "2019-12-16",
    operations: [AttributeParty, XmlMapsFlattenedNestedXmlNamespace]
}

@enum([{"value": "enumvalue", "name": "V"}])
string StringEnum

integer PrimitiveInt

structure AttributePartyInputOutput {
    @xmlAttribute
    enum: StringEnum,

    @xmlAttribute
    @xmlName("prefix:anumber")
    number: PrimitiveInt,

    @xmlAttribute
    ts: Timestamp,

    @xmlAttribute
    bool: Boolean
}

@httpResponseTests([{
        id: "DeserAttributes",
        code: 200,
        body: "<AttributePartyInputOutput enum=\"enumvalue\" prefix:anumber=\"5\" ts=\"1985-04-12T23:20:50.00Z\" bool=\"true\"/>",
        params: {
            enum: "enumvalue",
            number: 5,
            ts: 482196050,
            bool: true
        },
        protocol: "aws.protocols#restXml"

}])
@http(uri: "/AttributeParty", method: "POST")
operation AttributeParty {
    output: AttributePartyInputOutput
}

@httpResponseTests([{
        id: "DeserFlatNamespaceMaps",
        code: 200,
        body: "<XmlMapsFlattenedNestedXmlNamespaceInputOutput xmlns=\"http://aoo.com\"><myMap><yek xmlns=\"http://doo.com\">map2</yek><eulav xmlns=\"http://eoo.com\"><entry><K xmlns=\"http://goo.com\">third</K><V xmlns=\"http://hoo.com\">plz</V></entry><entry><K xmlns=\"http://goo.com\">fourth</K><V xmlns=\"http://hoo.com\">onegai</V></entry></eulav></myMap><myMap><yek xmlns=\"http://doo.com\">map1</yek><eulav xmlns=\"http://eoo.com\"><entry><K xmlns=\"http://goo.com\">second</K><V xmlns=\"http://hoo.com\">konnichiwa</V></entry><entry><K xmlns=\"http://goo.com\">first</K><V xmlns=\"http://hoo.com\">hi</V></entry></eulav></myMap></XmlMapsFlattenedNestedXmlNamespaceInput>",
        params: {
            "myMap": {
                "map2": {"fourth": "onegai", "third": "plz" },
                "map1": {"second": "konnichiwa", "first": "hi" }
            }
        },
        protocol: "aws.protocols#restXml"
}])
@http(uri: "/XmlMapsFlattenedNestedXmlNamespace", method: "POST")
operation XmlMapsFlattenedNestedXmlNamespace {
    input: XmlMapsFlattenedNestedXmlNamespaceInputOutput,
    output: XmlMapsFlattenedNestedXmlNamespaceInputOutput
}

@xmlNamespace(uri: "http://aoo.com")
structure XmlMapsFlattenedNestedXmlNamespaceInputOutput {
    @xmlNamespace(uri: "http://boo.com")
    @xmlFlattened
    myMap: XmlMapsNestedNamespaceInputOutputMap,
}

@xmlNamespace(uri: "http://coo.com")
map XmlMapsNestedNamespaceInputOutputMap {
    @xmlNamespace(uri: "http://doo.com")
    @xmlName("yek")
    key: String,

    @xmlNamespace(uri: "http://eoo.com")
    @xmlName("eulav")
    value: XmlMapsNestedNestedNamespaceInputOutputMap
}

@xmlNamespace(uri: "http://foo.com")
map XmlMapsNestedNestedNamespaceInputOutputMap {
    @xmlNamespace(uri: "http://goo.com")
    @xmlName("K")
    key: String,

    @xmlNamespace(uri: "http://hoo.com")
    @xmlName("V")
    value: String
}