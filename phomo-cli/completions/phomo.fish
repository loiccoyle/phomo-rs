complete -c phomo -s g -l grid-size -d 'Grid size, the number of tiles along the width and height' -r
complete -c phomo -s n -l n-appearances -d 'The number of times a tile can appear in the mosaic' -r
complete -c phomo -l solver -d 'The solver to use to compute the tile to cell assignments' -r -f -a "greedy\t''
auction\t''
hungarian\t''"
complete -c phomo -l metric -d 'The distance metric to use' -r -f -a "norm-l1\t''
norm-l2\t''
avg-color\t''
luminance-l1\t''
luminance-l2\t''"
complete -c phomo -l crop-tiles -d 'Crop tiles to grid cell size'
complete -c phomo -l resize-tiles -d 'Resize tiles to grid cell size'
complete -c phomo -l equalize -d 'Equalize the master and tile image color distributions'
complete -c phomo -l transfer-master-to-tiles -d 'Transfer the color palette of the master image to the tile images'
complete -c phomo -l transfer-tiles-to-master -d 'Transfer the color palette of the tile images to the master image'
complete -c phomo -s v -l verbose -d 'Increase logging verbosity'
complete -c phomo -s q -l quiet -d 'Decrease logging verbosity'
complete -c phomo -s h -l help -d 'Print help (see more with \'--help\')'
complete -c phomo -s V -l version -d 'Print version'
