<?php

// Not upstream: the class tree of nikic/php-parser 5 that the fixtures type values with (Rector's run loads it from
// its vendor directory), without members.

namespace PhpParser {
    interface Node
    {
    }

    abstract class NodeAbstract implements Node, \JsonSerializable
    {
    }
}

namespace PhpParser\Node {
    abstract class Expr extends \PhpParser\NodeAbstract
    {
    }

    abstract class Scalar extends Expr
    {
    }

    class Name extends \PhpParser\NodeAbstract
    {
    }
}

namespace PhpParser\Node\Expr {
    abstract class CallLike extends \PhpParser\Node\Expr
    {
    }

    class MethodCall extends CallLike
    {
    }

    class StaticCall extends CallLike
    {
    }
}

namespace PhpParser\Node\Scalar {
    class Int_ extends \PhpParser\Node\Scalar
    {
    }

    class String_ extends \PhpParser\Node\Scalar
    {
    }
}

namespace PhpParser\Node\Name {
    class FullyQualified extends \PhpParser\Node\Name
    {
    }
}
