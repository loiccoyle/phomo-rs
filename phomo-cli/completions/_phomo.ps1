
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'phomo' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'phomo'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'phomo' {
            [CompletionResult]::new('-g', '-g', [CompletionResultType]::ParameterName, 'Grid size, the number of tiles along the width and height')
            [CompletionResult]::new('--grid-size', '--grid-size', [CompletionResultType]::ParameterName, 'Grid size, the number of tiles along the width and height')
            [CompletionResult]::new('-n', '-n', [CompletionResultType]::ParameterName, 'The number of times a tile can appear in the mosaic')
            [CompletionResult]::new('--n-appearances', '--n-appearances', [CompletionResultType]::ParameterName, 'The number of times a tile can appear in the mosaic')
            [CompletionResult]::new('--solver', '--solver', [CompletionResultType]::ParameterName, 'The solver to use to compute the tile to cell assignments')
            [CompletionResult]::new('--metric', '--metric', [CompletionResultType]::ParameterName, 'The distance metric to use')
            [CompletionResult]::new('--crop-tiles', '--crop-tiles', [CompletionResultType]::ParameterName, 'Crop tiles to grid cell size')
            [CompletionResult]::new('--resize-tiles', '--resize-tiles', [CompletionResultType]::ParameterName, 'Resize tiles to grid cell size')
            [CompletionResult]::new('--equalize', '--equalize', [CompletionResultType]::ParameterName, 'Equalize the master and tile image color distributions')
            [CompletionResult]::new('--transfer-master-to-tiles', '--transfer-master-to-tiles', [CompletionResultType]::ParameterName, 'Transfer the color palette of the master image to the tile images')
            [CompletionResult]::new('--transfer-tiles-to-master', '--transfer-tiles-to-master', [CompletionResultType]::ParameterName, 'Transfer the color palette of the tile images to the master image')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Increase logging verbosity')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Decrease logging verbosity')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
