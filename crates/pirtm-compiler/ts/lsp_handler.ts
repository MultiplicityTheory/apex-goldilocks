import { Diagnostic, DiagnosticSeverity, PublishDiagnosticsParams } from 'vscode-languageserver';
import { DiagnosticEnvelope, DiagnosticReport, Severity } from './generated/pirtm_compiler'; // Generated via wasm-bindgen

/**
 * Maps the internal PIRTM Severity to LSP Severity.
 */
function mapSeverity(severity: Severity): DiagnosticSeverity {
    switch (severity) {
        case Severity.ERROR: return DiagnosticSeverity.Error;
        case Severity.WARNING: return DiagnosticSeverity.Warning;
        case Severity.ADVISORY: return DiagnosticSeverity.Information;
    }
}

/**
 * Maps a single DiagnosticReport to an LSP Diagnostic.
 */
function toLspDiagnostic(report: DiagnosticReport): Diagnostic {
    return {
        range: {
            start: { line: report.start_line, character: report.start_col },
            end: { line: report.end_line, character: report.end_col },
        },
        severity: mapSeverity(report.severity),
        code: report.code,
        message: report.message,
        source: 'PIRTM-Compiler'
    };
}

/**
 * Stateless handler to process validation output and publish to LSP client.
 */
export async function handleValidation(
    uri: string,
    rawEnvelope: any,
    publishDiagnostics: (params: PublishDiagnosticsParams) => void
): Promise<void> {
    // 1. Decode Envelope (Schema is canonical)
    const envelope = rawEnvelope as DiagnosticEnvelope;

    // 2. Map diagnostics (Direct 1:1 mapping, no reinterpretation)
    const diagnostics = envelope.diagnostics.map(toLspDiagnostic);

    // 3. Publish to client
    publishDiagnostics({
        uri,
        diagnostics
    });
}
