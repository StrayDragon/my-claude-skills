## Best Practices

### Diagram Design Principles

**Keep diagrams simple:**
- Limit to 15-20 nodes per diagram for readability
- Use clear, concise labels
- Group related elements together
- Use consistent styling

**Effective flowcharts:**
- Follow top-to-bottom or left-to-right flow
- Use decision points sparingly
- Color-code different types of actions
- Add meaningful annotations

**Sequence diagram tips:**
- Show clear message flow
- Use appropriate message types (sync, async, reply)
- Group related interactions
- Highlight important state changes

### Performance Optimization

**For web applications:**
```javascript
// Initialize Mermaid once
mermaid.initialize({
    startOnLoad: false,
    theme: 'default',
    themeVariables: {
        primaryColor: '#0066cc',
        primaryTextColor: '#333'
    }
});

// Render diagrams on demand
const renderDiagram = async (element, definition) => {
    const { svg } = await mermaid.render('mermaid-diagram', definition);
    element.innerHTML = svg;
};
```

### Accessibility Considerations

**Accessible Mermaid diagrams:**
- Provide alt text descriptions
- Use high-contrast colors
- Ensure keyboard navigation
- Include ARIA labels

```html
<div class="mermaid" aria-label="User authentication flowchart" role="img">
    flowchart TD
        A[Login] --> B{Valid?}
        B -->|Yes| C[Dashboard]
        B -->|No| D[Error]
</div>
```

## Advanced Features

### Custom Themes

**Define custom theme variables:**
```javascript
mermaid.initialize({
    theme: 'base',
    themeVariables: {
        primaryColor: '#ffcc00',
        primaryTextColor: '#333333',
        primaryBorderColor: '#333333',
        lineColor: '#333333',
        secondaryColor: '#fff',
        tertiaryColor: '#fff'
    }
});
```

### Interactive Features

**Click handlers:**
```javascript
// Add click actions to nodes
const definition = `
flowchart LR
    A[Click Me] --> B{Action}

    click A "https://example.com"
    click B "alert('Decision point')"
`;
```

