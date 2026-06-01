import { 
    createConnection, 
    ProposedFeatures, 
    TextDocuments, 
    TextDocumentSyncKind 
} from 'vscode-languageserver/node';
import { TextDocument } from 'vscode-languageserver-textdocument';
import { handleValidation } from './lsp_handler';
import * as pirtmCompiler from './generated/pirtm_compiler'; // WASM binding

// Create connection and documents
const connection = createConnection(ProposedFeatures.all);
const documents = new TextDocuments(TextDocument);

// Initialize WASM module
pirtmCompiler.default().then(() => {
    connection.console.info("PIRTM WASM Compiler initialized.");
});

// Sync handler
documents.onDidChangeContent(async (change) => {
    const text = change.document.getText();
    const uri = change.document.uri;
    
    // Call WASM binding (Assuming standard prime_set and stratum_id)
    const rawEnvelope = pirtmCompiler.validate_source(text, '{"2","3","5"}', 'strat-1');
    
    await handleValidation(uri, rawEnvelope, (params) => {
        connection.sendDiagnostics(params);
    });
});

documents.listen(connection);
connection.listen();
