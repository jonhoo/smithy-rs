/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

package software.amazon.smithy.rust.codegen.lang

import java.util.function.BiFunction
import software.amazon.smithy.codegen.core.CodegenException
import software.amazon.smithy.codegen.core.Symbol
import software.amazon.smithy.rust.codegen.smithy.RuntimeType
import software.amazon.smithy.rust.codegen.smithy.rustType
import software.amazon.smithy.utils.CodeWriter
import software.amazon.smithy.vended.CodegenWriter
import software.amazon.smithy.vended.CodegenWriterFactory

fun CodeWriter.withBlock(textBeforeNewLine: String, textAfterNewLine: String, block: CodeWriter.() -> Unit): CodeWriter {
    openBlock(textBeforeNewLine)
    block(this)
    closeBlock(textAfterNewLine)
    return this
}

/*
 * Writes a Rust-style block, demarcated by curly braces
 */
fun CodeWriter.rustBlock(header: String, vararg args: Any, block: CodeWriter.() -> Unit): CodeWriter {
    openBlock("$header {", *args)
    block(this)
    closeBlock("}")
    return this
}

class RustWriter(private val filename: String, private val namespace: String, private val commentCharacter: String = "//") : CodegenWriter<RustWriter, UseDeclarations>(null, UseDeclarations(filename, namespace)) {
    init {
        putFormatter('T', RustSymbolFormatter())
    }

    override fun toString(): String {
        val contents = super.toString()
        // hackity hack to support
        val header = "$commentCharacter Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT."
        val useDecls = importContainer.toString()
        return "$header\n$useDecls\n$contents\n"
    }

    inner class RustSymbolFormatter : BiFunction<Any, String, String> {
        override fun apply(t: Any, u: String): String {
            return when (t) {
                is RuntimeType -> {
                    t.dependency?.also { addDependency(it) }
                    // for now, use the fully qualified type name
                    "::${t.namespace}::${t.name}"
                }
                is Symbol -> {
                    if (t.namespace != namespace) {
                        addImport(t, null)
                    }
                    t.rustType().render()
                }
                else -> throw CodegenException("Invalid type provided to RustSymbolFormatter")
            }
        }
    }

    companion object {
        val Factory: CodegenWriterFactory<RustWriter> =
            CodegenWriterFactory<RustWriter> { filename, namespace -> when {
                filename.endsWith(".toml") -> RustWriter(filename, namespace, "#")
                else -> RustWriter(filename, namespace)
            } }
    }
}
