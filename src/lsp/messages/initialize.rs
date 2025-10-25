use serde::{Deserialize, Serialize};
use serde_json::Value;

use serde_repr::{Deserialize_repr, Serialize_repr};
// Helper macro for skipping None values
use serde_with::skip_serializing_none;

use crate::lsp::messages::core::{Id, ProgressToken, Request, Response};

// ============================================================================
// Main Initialize Types
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    // From WorkProgressParams
    pub work_done_token: Option<ProgressToken>,

    /// The process Id of the parent process that started the server.
    pub process_id: Option<i32>,

    /// Information about the client
    pub client_info: Option<ClientInfo>,

    /// The locale the client is currently showing the user interface in.
    pub locale: Option<String>,

    /// The rootPath of the workspace. Is null if no folder is open.
    /// @deprecated in favour of rootUri.
    pub root_path: Option<String>,

    /// The rootUri of the workspace. Is null if no folder is open.
    /// @deprecated in favour of workspaceFolders
    pub root_uri: Option<String>,

    /// User provided initialization options.
    pub initialization_options: Option<Value>,

    /// The capabilities provided by the client (editor or tool)
    pub capabilities: ClientCapabilities,

    /// The initial trace setting. If omitted trace is disabled ('off').
    pub trace: Option<TraceValue>,

    /// The workspace folders configured in the client when the server starts.
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientInfo {
    /// The name of the client as defined by the client.
    pub name: String,

    /// The client's version as defined by the client.
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TraceValue {
    Off,
    Messages,
    Verbose,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFolder {
    /// The associated URI for this workspace folder.
    pub uri: String,

    /// The name of the workspace folder. Used to refer to this
    /// workspace folder in the user interface.
    pub name: String,
}

// ============================================================================
// Client Capabilities
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientCapabilities {
    /// Workspace specific client capabilities.
    pub workspace: Option<WorkspaceClientCapabilities>,

    /// Text document specific client capabilities.
    pub text_document: Option<TextDocumentClientCapabilities>,

    /// Window specific client capabilities.
    pub window: Option<WindowClientCapabilities>,

    /// General client capabilities.
    pub general: Option<GeneralClientCapabilities>,

    /// Experimental client capabilities.
    pub experimental: Option<Value>,
}

// ============================================================================
// Workspace Capabilities
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceClientCapabilities {
    /// The client supports applying batch edits to the workspace.
    pub apply_edit: Option<bool>,

    /// Capabilities specific to `WorkspaceEdit`s
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,

    /// Capabilities specific to the `workspace/didChangeConfiguration` notification.
    pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,

    /// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
    pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,

    /// Capabilities specific to the `workspace/symbol` request.
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,

    /// Capabilities specific to the `workspace/executeCommand` request.
    pub execute_command: Option<ExecuteCommandClientCapabilities>,

    /// The client has support for workspace folders.
    pub workspace_folders: Option<bool>,

    /// The client supports `workspace/configuration` requests.
    pub configuration: Option<bool>,

    /// Capabilities specific to the semantic token requests scoped to the workspace.
    pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,

    /// Capabilities specific to the code lens requests scoped to the workspace.
    pub code_lens: Option<CodeLensWorkspaceClientCapabilities>,

    /// The client has support for file requests/notifications.
    pub file_operations: Option<FileOperationClientCapabilities>,

    /// Client workspace capabilities specific to inline values.
    pub inline_value: Option<InlineValueWorkspaceClientCapabilities>,

    /// Client workspace capabilities specific to inlay hints.
    pub inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,

    /// Client workspace capabilities specific to diagnostics.
    pub diagnostic: Option<DiagnosticWorkspaceClientCapabilities>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
    /// The client supports versioned document changes in `WorkspaceEdit`s
    pub document_changes: Option<bool>,

    /// The resource operations the client supports.
    pub resource_operations: Option<Vec<ResourceOperationKind>>,

    /// The failure handling strategy of a client if applying the workspace edit fails.
    pub failure_handling: Option<FailureHandlingKind>,

    /// Whether the client normalizes line endings to the client specific setting.
    pub normalizes_line_endings: Option<bool>,

    /// Whether the client in general supports change annotations on text edits.
    pub change_annotation_support: Option<ChangeAnnotationSupport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResourceOperationKind {
    Create,
    Rename,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FailureHandlingKind {
    Abort,
    Transactional,
    TextOnlyTransactional,
    Undo,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAnnotationSupport {
    /// Whether the client groups edits with equal labels into tree nodes.
    pub groups_on_label: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationClientCapabilities {
    /// Did change configuration notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
    /// Did change watched files notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Whether the client has support for relative patterns or not.
    pub relative_pattern_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceSymbolClientCapabilities {
    /// Symbol request supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
    pub symbol_kind: Option<SymbolKindCapability>,

    /// The client supports tags on `SymbolInformation`.
    pub tag_support: Option<TagSupport<SymbolTag>>,

    /// The client support partial workspace symbols.
    pub resolve_support: Option<ResolveSupport>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SymbolKindCapability {
    /// The symbol kind values the client supports.
    pub value_set: Option<Vec<SymbolKind>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SymbolKind {
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SymbolTag {
    Deprecated = 1,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TagSupport<T> {
    /// The tags supported by the client.
    pub value_set: Vec<T>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolveSupport {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandClientCapabilities {
    /// Execute command supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensWorkspaceClientCapabilities {
    /// Whether the client implementation supports a refresh request.
    pub refresh_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensWorkspaceClientCapabilities {
    /// Whether the client implementation supports a refresh request.
    pub refresh_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationClientCapabilities {
    /// Whether the client supports dynamic registration for file requests/notifications.
    pub dynamic_registration: Option<bool>,

    /// The client has support for sending didCreateFiles notifications.
    pub did_create: Option<bool>,

    /// The client has support for sending willCreateFiles requests.
    pub will_create: Option<bool>,

    /// The client has support for sending didRenameFiles notifications.
    pub did_rename: Option<bool>,

    /// The client has support for sending willRenameFiles requests.
    pub will_rename: Option<bool>,

    /// The client has support for sending didDeleteFiles notifications.
    pub did_delete: Option<bool>,

    /// The client has support for sending willDeleteFiles requests.
    pub will_delete: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueWorkspaceClientCapabilities {
    /// Whether the client implementation supports a refresh request.
    pub refresh_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintWorkspaceClientCapabilities {
    /// Whether the client implementation supports a refresh request.
    pub refresh_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticWorkspaceClientCapabilities {
    /// Whether the client implementation supports a refresh request.
    pub refresh_support: Option<bool>,
}

// ============================================================================
// Text Document Capabilities
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,

    /// Capabilities specific to the `textDocument/completion` request.
    pub completion: Option<CompletionClientCapabilities>,

    /// Capabilities specific to the `textDocument/hover` request.
    pub hover: Option<HoverClientCapabilities>,

    /// Capabilities specific to the `textDocument/signatureHelp` request.
    pub signature_help: Option<SignatureHelpClientCapabilities>,

    /// Capabilities specific to the `textDocument/declaration` request.
    pub declaration: Option<DeclarationClientCapabilities>,

    /// Capabilities specific to the `textDocument/definition` request.
    pub definition: Option<DefinitionClientCapabilities>,

    /// Capabilities specific to the `textDocument/typeDefinition` request.
    pub type_definition: Option<TypeDefinitionClientCapabilities>,

    /// Capabilities specific to the `textDocument/implementation` request.
    pub implementation: Option<ImplementationClientCapabilities>,

    /// Capabilities specific to the `textDocument/references` request.
    pub references: Option<ReferenceClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentHighlight` request.
    pub document_highlight: Option<DocumentHighlightClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentSymbol` request.
    pub document_symbol: Option<DocumentSymbolClientCapabilities>,

    /// Capabilities specific to the `textDocument/codeAction` request.
    pub code_action: Option<CodeActionClientCapabilities>,

    /// Capabilities specific to the `textDocument/codeLens` request.
    pub code_lens: Option<CodeLensClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentLink` request.
    pub document_link: Option<DocumentLinkClientCapabilities>,

    /// Capabilities specific to the `textDocument/documentColor` and the
    /// `textDocument/colorPresentation` request.
    pub color_provider: Option<DocumentColorClientCapabilities>,

    /// Capabilities specific to the `textDocument/formatting` request.
    pub formatting: Option<DocumentFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/rangeFormatting` request.
    pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/onTypeFormatting` request.
    pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,

    /// Capabilities specific to the `textDocument/rename` request.
    pub rename: Option<RenameClientCapabilities>,

    /// Capabilities specific to the `textDocument/publishDiagnostics` notification.
    pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,

    /// Capabilities specific to the `textDocument/foldingRange` request.
    pub folding_range: Option<FoldingRangeClientCapabilities>,

    /// Capabilities specific to the `textDocument/selectionRange` request.
    pub selection_range: Option<SelectionRangeClientCapabilities>,

    /// Capabilities specific to the `textDocument/linkedEditingRange` request.
    pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,

    /// Capabilities specific to the various call hierarchy requests.
    pub call_hierarchy: Option<CallHierarchyClientCapabilities>,

    /// Capabilities specific to the various semantic token requests.
    pub semantic_tokens: Option<SemanticTokensClientCapabilities>,

    /// Capabilities specific to the `textDocument/moniker` request.
    pub moniker: Option<MonikerClientCapabilities>,

    /// Capabilities specific to the various type hierarchy requests.
    pub type_hierarchy: Option<TypeHierarchyClientCapabilities>,

    /// Capabilities specific to the `textDocument/inlineValue` request.
    pub inline_value: Option<InlineValueClientCapabilities>,

    /// Capabilities specific to the `textDocument/inlayHint` request.
    pub inlay_hint: Option<InlayHintClientCapabilities>,

    /// Capabilities specific to the diagnostic pull model.
    pub diagnostic: Option<DiagnosticClientCapabilities>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
    /// Whether text document synchronization supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports sending will save notifications.
    pub will_save: Option<bool>,

    /// The client supports sending a will save request and
    /// waits for a response providing text edits which will
    /// be applied to the document before it is saved.
    pub will_save_wait_until: Option<bool>,

    /// The client supports did save notifications.
    pub did_save: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionClientCapabilities {
    /// Whether completion supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports the following `CompletionItem` specific capabilities.
    pub completion_item: Option<CompletionItemCapability>,

    pub completion_item_kind: Option<CompletionItemKindCapability>,

    /// The client supports to send additional context information for a
    /// `textDocument/completion` request.
    pub context_support: Option<bool>,

    /// The client's default when the completion item doesn't provide an
    /// `insertTextMode` property.
    pub insert_text_mode: Option<InsertTextMode>,

    /// The client supports the following `CompletionList` specific capabilities.
    pub completion_list: Option<CompletionListCapability>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemCapability {
    /// Client supports snippets as insert text.
    pub snippet_support: Option<bool>,

    /// Client supports commit characters on a completion item.
    pub commit_characters_support: Option<bool>,

    /// Client supports the following content formats for the documentation
    /// property.
    pub documentation_format: Option<Vec<MarkupKind>>,

    /// Client supports the deprecated property on a completion item.
    pub deprecated_support: Option<bool>,

    /// Client supports the preselect property on a completion item.
    pub preselect_support: Option<bool>,

    /// Client supports the tag property on a completion item.
    pub tag_support: Option<TagSupport<CompletionItemTag>>,

    /// Client support insert replace edit to control different behavior if a
    /// completion item is inserted in the text or should replace text.
    pub insert_replace_support: Option<bool>,

    /// Indicates which properties a client can resolve lazily on a completion
    /// item.
    pub resolve_support: Option<ResolveSupport>,

    /// The client supports the `insertTextMode` property on a completion item.
    pub insert_text_mode_support: Option<InsertTextModeSupport>,

    /// The client has support for completion item label details.
    pub label_details_support: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MarkupKind {
    #[serde(rename = "plaintext")]
    PlainText,
    #[serde(rename = "markdown")]
    Markdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CompletionItemTag {
    Deprecated = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum InsertTextMode {
    AsIs = 1,
    AdjustIndentation = 2,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InsertTextModeSupport {
    pub value_set: Vec<InsertTextMode>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItemKindCapability {
    /// The completion item kind values the client supports.
    pub value_set: Option<Vec<CompletionItemKind>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum CompletionItemKind {
    Text = 1,
    Method = 2,
    Function = 3,
    Constructor = 4,
    Field = 5,
    Variable = 6,
    Class = 7,
    Interface = 8,
    Module = 9,
    Property = 10,
    Unit = 11,
    Value = 12,
    Enum = 13,
    Keyword = 14,
    Snippet = 15,
    Color = 16,
    File = 17,
    Reference = 18,
    Folder = 19,
    EnumMember = 20,
    Constant = 21,
    Struct = 22,
    Event = 23,
    Operator = 24,
    TypeParameter = 25,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompletionListCapability {
    /// The client supports the following itemDefaults on a completion list.
    pub item_defaults: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    /// Whether hover supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Client supports the following content formats for the content property.
    pub content_format: Option<Vec<MarkupKind>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpClientCapabilities {
    /// Whether signature help supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports the following `SignatureInformation` specific properties.
    pub signature_information: Option<SignatureInformationCapability>,

    /// The client supports to send additional context information for a
    /// `textDocument/signatureHelp` request.
    pub context_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignatureInformationCapability {
    /// Client supports the following content formats for the documentation
    /// property.
    pub documentation_format: Option<Vec<MarkupKind>>,

    /// Client capabilities specific to parameter information.
    pub parameter_information: Option<ParameterInformationCapability>,

    /// The client supports the `activeParameter` property on `SignatureInformation`.
    pub active_parameter_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParameterInformationCapability {
    /// The client supports processing label offsets instead of a simple label string.
    pub label_offset_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationClientCapabilities {
    /// Whether declaration supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports additional metadata in the form of declaration links.
    pub link_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefinitionClientCapabilities {
    /// Whether definition supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports additional metadata in the form of definition links.
    pub link_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionClientCapabilities {
    /// Whether type definition supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports additional metadata in the form of definition links.
    pub link_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationClientCapabilities {
    /// Whether implementation supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports additional metadata in the form of definition links.
    pub link_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceClientCapabilities {
    /// Whether references supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentHighlightClientCapabilities {
    /// Whether document highlight supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentSymbolClientCapabilities {
    /// Whether document symbol supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Specific capabilities for the `SymbolKind`.
    pub symbol_kind: Option<SymbolKindCapability>,

    /// The client supports hierarchical document symbols.
    pub hierarchical_document_symbol_support: Option<bool>,

    /// The client supports tags on `SymbolInformation`.
    pub tag_support: Option<TagSupport<SymbolTag>>,

    /// The client supports an additional label presented in the UI when
    /// registering a document symbol provider.
    pub label_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionClientCapabilities {
    /// Whether code action supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The client supports code action literals as a valid
    /// response of the `textDocument/codeAction` request.
    pub code_action_literal_support: Option<CodeActionLiteralSupport>,

    /// Whether code action supports the `isPreferred` property.
    pub is_preferred_support: Option<bool>,

    /// Whether code action supports the `disabled` property.
    pub disabled_support: Option<bool>,

    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/codeAction` and a
    /// `codeAction/resolve` request.
    pub data_support: Option<bool>,

    /// Whether the client supports resolving additional code action
    /// properties via a separate `codeAction/resolve` request.
    pub resolve_support: Option<ResolveSupport>,

    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// `CodeAction#edit` property by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    pub honors_change_annotations: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionLiteralSupport {
    /// The code action kind is supported with the following value set.
    pub code_action_kind: CodeActionKindCapability,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionKindCapability {
    /// The code action kind values the client supports.
    pub value_set: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensClientCapabilities {
    /// Whether code lens supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentLinkClientCapabilities {
    /// Whether document link supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Whether the client supports the `tooltip` property on `DocumentLink`.
    pub tooltip_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorClientCapabilities {
    /// Whether document color supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFormattingClientCapabilities {
    /// Whether formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentRangeFormattingClientCapabilities {
    /// Whether range formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentOnTypeFormattingClientCapabilities {
    /// Whether on type formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientCapabilities {
    /// Whether rename supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Client supports testing for validity of rename operations
    /// before execution.
    pub prepare_support: Option<bool>,

    /// Client supports the default behavior result.
    pub prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,

    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// rename request's workspace edit by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    pub honors_change_annotations: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PrepareSupportDefaultBehavior {
    Identifier = 1,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsClientCapabilities {
    /// Whether the clients accepts diagnostics with related information.
    pub related_information: Option<bool>,

    /// Client supports the tag property to provide meta data about a diagnostic.
    pub tag_support: Option<TagSupport<DiagnosticTag>>,

    /// Whether the client interprets the version property of the
    /// `textDocument/publishDiagnostics` notification's parameter.
    pub version_support: Option<bool>,

    /// Client supports a codeDescription property
    pub code_description_support: Option<bool>,

    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/publishDiagnostics` and
    /// `textDocument/codeAction` request.
    pub data_support: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DiagnosticTag {
    Unnecessary = 1,
    Deprecated = 2,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeClientCapabilities {
    /// Whether folding range supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// The maximum number of folding ranges that the client prefers to receive
    /// per document.
    pub range_limit: Option<u32>,

    /// If set, the client signals that it only supports folding complete lines.
    pub line_folding_only: Option<bool>,

    /// Specific options for the folding range kind.
    pub folding_range_kind: Option<FoldingRangeKindCapability>,

    /// Specific options for the folding range.
    pub folding_range: Option<FoldingRangeCapability>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeKindCapability {
    /// The folding range kind values the client supports.
    pub value_set: Option<Vec<String>>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeCapability {
    /// If set, the client signals that it supports setting collapsedText on
    /// folding ranges to display custom labels instead of the default text.
    pub collapsed_text: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeClientCapabilities {
    /// Whether selection range supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeClientCapabilities {
    /// Whether linked editing range supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallHierarchyClientCapabilities {
    /// Whether call hierarchy supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensClientCapabilities {
    /// Whether semantic tokens supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Which requests the client supports and might send to the server.
    pub requests: SemanticTokensRequestsCapability,

    /// The token types that the client supports.
    pub token_types: Vec<String>,

    /// The token modifiers that the client supports.
    pub token_modifiers: Vec<String>,

    /// The formats the clients supports.
    pub formats: Vec<TokenFormat>,

    /// Whether the client supports tokens that can overlap each other.
    pub overlapping_token_support: Option<bool>,

    /// Whether the client supports tokens that can span multiple lines.
    pub multiline_token_support: Option<bool>,

    /// Whether the client allows the server to actively cancel a
    /// semantic token request.
    pub server_cancel_support: Option<bool>,

    /// Whether the client uses semantic tokens to augment existing
    /// syntax tokens.
    pub augments_syntax_tokens: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensRequestsCapability {
    /// The client will send the `textDocument/semanticTokens/range` request.
    pub range: Option<RangeCapability>,

    /// The client will send the `textDocument/semanticTokens/full` request.
    pub full: Option<FullCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeCapability {
    Bool(bool),
    Object {},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FullCapability {
    Bool(bool),
    Object { delta: Option<bool> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TokenFormat {
    Relative,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MonikerClientCapabilities {
    /// Whether moniker supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TypeHierarchyClientCapabilities {
    /// Whether type hierarchy supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlineValueClientCapabilities {
    /// Whether inline value supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InlayHintClientCapabilities {
    /// Whether inlay hint supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Indicates which properties a client can resolve lazily on an inlay hint.
    pub resolve_support: Option<ResolveSupport>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticClientCapabilities {
    /// Whether diagnostic supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    /// Whether the clients supports related documents for document diagnostic pulls.
    pub related_document_support: Option<bool>,
}

// ============================================================================
// Window Capabilities
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WindowClientCapabilities {
    /// Whether client supports handling progress notifications.
    pub work_done_progress: Option<bool>,

    /// Capabilities specific to the showMessage request.
    pub show_message: Option<ShowMessageRequestClientCapabilities>,

    /// Capabilities specific to the showDocument request.
    pub show_document: Option<ShowDocumentClientCapabilities>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestClientCapabilities {
    /// Capabilities specific to the `MessageActionItem` type.
    pub message_action_item: Option<MessageActionItemCapability>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItemCapability {
    /// Whether the client supports additional attributes which
    /// are preserved and send back to the server in the
    /// request's response.
    pub additional_properties_support: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShowDocumentClientCapabilities {
    /// The client has support for the show document request.
    pub support: bool,
}

// ============================================================================
// General Capabilities
// ============================================================================

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GeneralClientCapabilities {
    /// Client capabilities specific to regular expressions.
    pub regular_expressions: Option<RegularExpressionsClientCapabilities>,

    /// Client capabilities specific to the client's markdown parser.
    pub markdown: Option<MarkdownClientCapabilities>,

    /// The position encodings supported by the client.
    pub position_encodings: Option<Vec<PositionEncodingKind>>,

    /// Client capability that signals how the client handles stale requests.
    pub stale_request_support: Option<StaleRequestSupport>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RegularExpressionsClientCapabilities {
    /// The engine's name.
    pub engine: String,

    /// The engine's version.
    pub version: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownClientCapabilities {
    /// The name of the parser.
    pub parser: String,

    /// The version of the parser.
    pub version: Option<String>,

    /// A list of HTML tags that the client allows / supports in Markdown.
    pub allowed_tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    Utf8,
    #[serde(rename = "utf-16")]
    Utf16,
    #[serde(rename = "utf-32")]
    Utf32,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StaleRequestSupport {
    /// The client will actively cancel the request.
    pub cancel: bool,

    /// The list of requests for which the client
    /// will retry the request if it receives a
    /// response with error code `ContentModified`
    pub retry_on_content_modified: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    /// Information about the server.
    pub server_info: Option<ServerInfo>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    /// The server's version as defined by the server.
    pub version: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    /// Defines how text documents are synced.
    pub text_document_sync: Option<TextDocumentSyncCapability>,

    /// The server provides hover support.
    pub hover_provider: Option<HoverProviderCapability>,

    /// The server provides completion support.
    pub completion_provider: Option<CompletionOptions>,

    /// The server provides signature help support.
    pub signature_help_provider: Option<SignatureHelpOptions>,

    /// The server provides goto definition support.
    pub definition_provider: Option<bool>,

    /// The server provides goto type definition support.
    pub type_definition_provider: Option<bool>,

    /// The server provides goto implementation support.
    pub implementation_provider: Option<bool>,

    /// The server provides find references support.
    pub references_provider: Option<bool>,

    /// The server provides document highlight support.
    pub document_highlight_provider: Option<bool>,

    /// The server provides document symbol support.
    pub document_symbol_provider: Option<bool>,

    /// The server provides code action support.
    pub code_action_provider: Option<CodeActionProviderCapability>,

    /// The server provides code lens support.
    pub code_lens_provider: Option<CodeLensOptions>,

    /// The server provides document formatting.
    pub document_formatting_provider: Option<bool>,

    /// The server provides document range formatting.
    pub document_range_formatting_provider: Option<bool>,

    /// The server provides rename support.
    pub rename_provider: Option<RenameProviderCapability>,

    /// The server provides folding provider support.
    pub folding_range_provider: Option<bool>,

    /// The server provides execute command support.
    pub execute_command_provider: Option<ExecuteCommandOptions>,

    /// The server provides workspace symbol support.
    pub workspace_symbol_provider: Option<bool>,

    /// Workspace specific server capabilities
    pub workspace: Option<WorkspaceServerCapabilities>,

    /// Semantic tokens provider capabilities.
    pub semantic_tokens_provider: Option<SemanticTokensOptions>,

    /// Experimental server capabilities.
    pub experimental: Option<Value>,
}

// Text document sync
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSyncCapability {
    Kind(TextDocumentSyncKind),
    Options(TextDocumentSyncOptions),
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    pub open_close: Option<bool>,
    pub change: Option<TextDocumentSyncKind>,
    pub will_save: Option<bool>,
    pub will_save_wait_until: Option<bool>,
    pub save: Option<SaveOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveOptions {
    pub include_text: Option<bool>,
}

// Hover
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverProviderCapability {
    Simple(bool),
    Options(HoverOptions),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverOptions {
    pub work_done_progress: Option<bool>,
}

// Completion
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionOptions {
    pub trigger_characters: Option<Vec<String>>,
    pub all_commit_characters: Option<Vec<String>>,
    pub resolve_provider: Option<bool>,
    pub work_done_progress: Option<bool>,
}

// Signature Help
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHelpOptions {
    pub trigger_characters: Option<Vec<String>>,
    pub retrigger_characters: Option<Vec<String>>,
    pub work_done_progress: Option<bool>,
}

// Code Action
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionProviderCapability {
    Simple(bool),
    Options(CodeActionOptions),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeActionOptions {
    pub code_action_kinds: Option<Vec<String>>,
    pub resolve_provider: Option<bool>,
    pub work_done_progress: Option<bool>,
}

// Code Lens
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeLensOptions {
    pub resolve_provider: Option<bool>,
    pub work_done_progress: Option<bool>,
}

// Rename
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameProviderCapability {
    Simple(bool),
    Options(RenameOptions),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameOptions {
    pub prepare_provider: Option<bool>,
    pub work_done_progress: Option<bool>,
}

// Execute Command
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteCommandOptions {
    pub commands: Vec<String>,
    pub work_done_progress: Option<bool>,
}

// Workspace
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceServerCapabilities {
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,
    pub file_operations: Option<FileOperationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersServerCapabilities {
    pub supported: Option<bool>,
    pub change_notifications: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileOperationOptions {
    pub did_create: Option<FileOperationRegistrationOptions>,
    pub will_create: Option<FileOperationRegistrationOptions>,
    pub did_rename: Option<FileOperationRegistrationOptions>,
    pub will_rename: Option<FileOperationRegistrationOptions>,
    pub did_delete: Option<FileOperationRegistrationOptions>,
    pub will_delete: Option<FileOperationRegistrationOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperationRegistrationOptions {
    pub filters: Vec<FileOperationFilter>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperationFilter {
    pub pattern: FileOperationPattern,
    pub scheme: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct FileOperationPattern {
    pub glob: String,
    pub matches: Option<String>,
    pub options: Option<Value>,
}

// Semantic Tokens
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensOptions {
    pub legend: SemanticTokensLegend,
    pub range: Option<bool>,
    pub full: Option<SemanticTokensFullOptions>,
    pub work_done_progress: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticTokensLegend {
    pub token_types: Vec<String>,
    pub token_modifiers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensFullOptions {
    Bool(bool),
    Delta { delta: Option<bool> },
}

#[derive(Debug, Deserialize)]
pub struct JustLspInitializeRequest {
    #[serde(flatten)]
    pub request: Request,
    pub params: InitializeParams,
}

#[derive(Debug, Serialize)]
#[skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct JustLspInitializeResponse {
    #[serde(flatten)]
    pub response: Response,
    pub result: InitializeResult,
}

impl JustLspInitializeResponse {
    pub fn new(id: String) -> Self {
        Self {
            response: Response {
                jsonrpc: String::from("2.0"),
                id: Some(Id::String(id)),
                result: None,
                error: None,
            },
            result: InitializeResult {
                capabilities: ServerCapabilities::default(),
                server_info: Some(ServerInfo {
                    name: String::from("just_a_language_server"),
                    version: Some(String::from("0.0.0")),
                }),
            },
        }
    }
}
