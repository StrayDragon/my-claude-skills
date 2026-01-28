## Common Extension Templates

### Basic Command Extension
```typescript
import * as vscode from 'vscode';

export function activate(context: vscode.ExtensionContext) {
    const disposable = vscode.commands.registerCommand('extension.myCommand', () => {
        vscode.window.showInformationMessage('Command executed!');
    });

    context.subscriptions.push(disposable);
}
```

### Language Server Extension
```typescript
import * as vscode from 'vscode';
import { LanguageClient, LanguageClientOptions } from 'vscode-languageclient/node';

export function activate(context: vscode.ExtensionContext) {
    const serverOptions: ServerOptions = {
        command: 'path/to/server',
        args: []
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'mylang' }]
    };

    const client = new LanguageClient(
        'myLanguageServer',
        'My Language Server',
        serverOptions,
        clientOptions
    );

    client.start();
}
```

