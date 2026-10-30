<?php

// Not upstream: nikic/php-parser 5's `Node` interface and `Stmt\If_` class (which implements it through `Stmt` and
// `NodeAbstract`), which a fixture names and Rector's run loads from its vendor directory.

declare(strict_types=1);

namespace PhpParser;

interface Node
{
}
