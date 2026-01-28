## Advanced Topics

### 1. Multi-root Workspace Support

```typescript
// Handle multi-root workspaces
function getWorkspaceFolders() {
    const folders = vscode.workspace.workspaceFolders;
    if (!folders) {
        vscode.window.showErrorMessage('No folder opened');
        return [];
    }
    return folders;
}

// Listen to workspace folder changes
vscode.workspace.onDidChangeWorkspaceFolders(event => {
    event.added.forEach(folder => {
        console.log(`Folder added: ${folder.name}`);
    });
    event.removed.forEach(folder => {
        console.log(`Folder removed: ${folder.name}`);
    });
});
```

### 2. Terminal Integration

**Create Terminal**
```typescript
export function createTerminal() {
    const terminal = vscode.window.createTerminal({
        name: 'My Extension Terminal',
        cwd: vscode.workspace.workspaceFolders?.[0]?.uri.fsPath
    });

    terminal.sendText('echo "Hello from terminal!"');
    terminal.show();
    return terminal;
}

// Terminal process provider (for custom shells)
export class CustomTerminal implements vscode.Pseudoterminal {
    private writeEmitter = new vscode.EventEmitter<string>();
    onDidWrite: vscode.Event<string> = this.writeEmitter.event;

    private closeEmitter = new vscode.EventEmitter<number>();
    onDidClose?: vscode.Event<number> = this.closeEmitter.event;

    private open = false;

    open(): void {
        this.open = true;
        this.writeEmitter.fire('Custom terminal started\r\n');
    }

    close(): void {
        this.open = false;
        this.closeEmitter.fire(0);
    }

    handleInput(data: string): void {
        if (data === '\r' || data === '\n') {
            this.writeEmitter.fire('\r\n');
        } else {
            this.writeEmitter.fire(data);
        }
    }
}
```

### 3. File System Provider

```typescript
export class CustomFileSystemProvider implements vscode.FileSystemProvider {
    private _onDidChangeFile: vscode.EventEmitter<vscode.FileChangeEvent[]> = new vscode.EventEmitter<vscode.FileChangeEvent[]>();
    onDidChangeFile: vscode.Event<vscode.FileChangeEvent[]> = this._onDidChangeFile.event;

    watch(uri: vscode.Uri): vscode.Disposable {
        // Return a disposable that stops watching
        return new vscode.Disposable(() => {});
    }

    stat(uri: vscode.Uri): vscode.FileStat | Thenable<vscode.FileStat> {
        return {
            type: vscode.FileType.File,
            ctime: Date.now(),
            mtime: Date.now(),
            size: 1024
        };
    }

    readDirectory(uri: vscode.Uri): [string, vscode.FileType][] | Thenable<[string, vscode.FileType][]> {
        return [
            ['file1.txt', vscode.FileType.File],
            ['folder1', vscode.FileType.Directory]
        ];
    }

    readFile(uri: vscode.Uri): Uint8Array | Thenable<Uint8Array> {
        return new Uint8Array(Buffer.from('Hello from custom file system'));
    }

    writeFile(uri: vscode.Uri, content: Uint8Array, options: { create: boolean; overwrite: boolean; }): void | Thenable<void> {
        // Handle file writing
    }

    delete(uri: vscode.Uri): void | Thenable<void> {
        // Handle file deletion
    }

    rename(oldUri: vscode.Uri, newUri: vscode.Uri, options: { overwrite: boolean; }): void | Thenable<void> {
        // Handle file renaming
    }

    createDirectory(uri: vscode.Uri): void | Thenable<void> {
        // Handle directory creation
    }
}
```

## Best Practices

### 1. Performance Optimization

**Lazy Loading**
```typescript
export function activate(context: vscode.ExtensionContext) {
    // Lazy load expensive modules
    const getExpensiveModule = () => {
        return require('./expensive-module');
    };

    const disposable = vscode.commands.registerCommand('extension.expensiveCommand', () => {
        const module = getExpensiveModule();
        module.doExpensiveOperation();
    });

    context.subscriptions.push(disposable);
}
```

**Efficient Data Structures**
```typescript
// Use efficient data structures for large datasets
class EfficientDataManager {
    private cache = new Map<string, any>();
    private disposables: vscode.Disposable[] = [];

    constructor() {
        // Clean up cache when documents change
        this.disposables.push(
            vscode.workspace.onDidChangeTextDocument(event => {
                this.cache.delete(event.document.uri.toString());
            })
        );
    }

    getData(uri: vscode.Uri): any {
        const key = uri.toString();
        if (!this.cache.has(key)) {
            this.cache.set(key, this.computeData(uri));
        }
        return this.cache.get(key);
    }

    dispose() {
        this.disposables.forEach(d => d.dispose());
    }
}
```

### 2. Error Handling

**Robust Error Handling**
```typescript
export async function safeCommand() {
    try {
        const result = await vscode.window.showInputBox({
            prompt: 'Enter something',
            validateInput: value => {
                if (!value || value.trim() === '') {
                    return 'Please enter a valid value';
                }
                return null;
            }
        });

        if (result) {
            // Process input
            vscode.window.showInformationMessage(`Processing: ${result}`);
        }
    } catch (error) {
        console.error('Command failed:', error);
        vscode.window.showErrorMessage(`Command failed: ${error.message}`);
    }
}
```

### 3. User Experience

**Progress Indicators**
```typescript
export async function longRunningOperation() {
    await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: 'Long Running Operation',
        cancellable: true
    }, async (progress, token) => {
        for (let i = 0; i < 100; i++) {
            if (token.isCancellationRequested) {
                vscode.window.showInformationMessage('Operation cancelled');
                return;
            }

            progress.report({
                increment: 1,
                message: `Processing item ${i + 1}/100`
            });

            // Simulate work
            await new Promise(resolve => setTimeout(resolve, 100));
        }
    });

    vscode.window.showInformationMessage('Operation completed!');
}
```

