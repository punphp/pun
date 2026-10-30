<?php

// The nikic/php-parser 5 classes Rector's fixtures name, declared as php-parser declares them (members trimmed).

namespace PhpParser {
    interface Node
    {
    }

    abstract class NodeAbstract implements Node, \JsonSerializable
    {
    }

    interface NodeVisitor
    {
        public function beforeTraverse(array $nodes);

        public function enterNode(Node $node);

        public function leaveNode(Node $node);

        public function afterTraverse(array $nodes);
    }

    abstract class NodeVisitorAbstract implements NodeVisitor
    {
        public function beforeTraverse(array $nodes)
        {
            return null;
        }

        public function enterNode(Node $node)
        {
            return null;
        }

        public function leaveNode(Node $node)
        {
            return null;
        }

        public function afterTraverse(array $nodes)
        {
            return null;
        }
    }
}

namespace PhpParser\Node {
    abstract class Expr extends \PhpParser\NodeAbstract
    {
    }

    abstract class Scalar extends Expr
    {
    }

    abstract class Stmt extends \PhpParser\NodeAbstract
    {
    }
}

namespace PhpParser\Node\Scalar {
    class Int_ extends \PhpParser\Node\Scalar
    {
        public function __construct(int $value, array $attributes = [])
        {
        }
    }
}
