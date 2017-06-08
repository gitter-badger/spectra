use std::collections::{HashMap, HashSet};

/// A shader module.
///
/// A shader module is a container that associates some shading code to several identifiers.
struct ShaderModule {
  symbols: HashMap<Identifier, ShadingCode>
}

/// Spectra Shading Language AST.
#[derive(Clone, Debug, Eq, PartialEq)]
enum SSL {
  /// An `export list_of_identifiers_` statement.
  Export(ExportList),
  /// A `from module use list of identifiers` statement.
  FromUse(ImportList),
  /// A `pipeline { list_of_pipeline_attributes }` statement.
  Pipeline(PipelineStatement),
  /// A yield statement, valid in geometry shaders.
  Yield(GeometryYieldExpression),
}

/// A module.
type ModuleName = String;
/// An identifier.
type Identifier = String;
/// Some opaque shading code.
type ShadingCode = String;
/// An expression.
type Expression = String;

/// An export non-empty list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportList {
  pub export_list: HashSet<ModulePath>
}

/// An import non-empty list.
#[derive(Clone, Debug, Eq, PartialEq)]
struct ImportList {
  module: ModuleName,
  list: HashSet<ModulePath>
}

/// A module path is a list of module(s), representing a hierarchy.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ModulePath {
  hierarchy: Vec<ModuleName>
}

/// A pipeline statement.
#[derive(Clone, Debug, Eq, PartialEq)]
struct PipelineStatement {
  attributes: Vec<PipelineAttribute>
}

/// Attributes that can be set in a pipeline.
#[derive(Clone, Debug, Eq, PartialEq)]
enum PipelineAttribute {
  /// Maximum vertices that the geometry shader can output.
  GeometryShaderMaxVertices(u32),
  /// Number of times the geometry shader must be invoked.
  GeometryShaderInvokations(u32)
}

/// Expressions that can be yielded in a geometry shader.
#[derive(Clone, Debug, Eq, PartialEq)]
enum GeometryYieldExpression {
  /// Yield a primitive.
  YieldPrimitive,
  /// Yield a primitive’s vertex (fold vertex).
  YieldFoldVertex(Expression)
}

/// Error that can occur when parsing SSL code.
#[derive(Clone, Debug, Eq, PartialEq)]
enum ParseError {
  ExpressionError(String)
}
