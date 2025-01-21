
use builtin;
use str;

set edit:completion:arg-completer[phomo] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'phomo'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'phomo'= {
            cand -g 'Grid size, the number of tiles along the width and height'
            cand --grid-size 'Grid size, the number of tiles along the width and height'
            cand -n 'The number of times a tile can appear in the mosaic'
            cand --n-appearances 'The number of times a tile can appear in the mosaic'
            cand --metric 'The distance metric to use'
            cand --crop-tiles 'Crop tiles to grid cell size'
            cand --resize-tiles 'Resize tiles to grid cell size'
            cand --equalize 'Equalize the master and tile image color distributions'
            cand --transfer-master-to-tiles 'Transfer the color palette of the master image to the tile images'
            cand --transfer-tiles-to-master 'Transfer the color palette of the tile images to the master image'
            cand --greedy 'Use a greedy tile assignment algorithm. Should improve performance at the expense of accuracy'
            cand -v 'Increase logging verbosity'
            cand --verbose 'Increase logging verbosity'
            cand -q 'Decrease logging verbosity'
            cand --quiet 'Decrease logging verbosity'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
