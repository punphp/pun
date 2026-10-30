<?php

// Rector stops on `view(...)` with "System error: Invalid value" (NodeFactory::createArgs takes no
// VariadicPlaceholder). Pun leaves a first-class callable alone.

$f = view(...);
