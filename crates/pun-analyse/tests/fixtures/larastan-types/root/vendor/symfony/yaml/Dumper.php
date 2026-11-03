<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Yaml;

use Symfony\Component\Yaml\Tag\TaggedValue;

/**
 * Dumper dumps PHP variables to YAML strings.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 *
 * @final
 */
class Dumper
{
    /**
     * @param int $indentation The amount of spaces to use for indentation of nested nodes
     */
    public function __construct(private int $indentation = 4)
    {}

    /**
     * Dumps a PHP value to YAML.
     *
     * @param mixed                     $input  The PHP value
     * @param int                       $inline The level where you switch to inline YAML
     * @param int                       $indent The level of indentation (used internally)
     * @param int-mask-of<Yaml::DUMP_*> $flags  A bit field of Yaml::DUMP_* constants to customize the dumped YAML string
     */
    public function dump(mixed $input, int $inline = 0, int $indent = 0, int $flags = 0): string
    {}

    private function doDump(mixed $input, int $inline = 0, int $indent = 0, int $flags = 0, int $nestingLevel = 0): string
    {}

    private function dumpTaggedValue(TaggedValue $value, int $inline, int $indent, int $flags, string $prefix, int $nestingLevel): string
    {}

    private function getBlockChompingIndicator(string $value): string
    {}

    private function dumpBlockScalarLines(string $value, string $prefix): string
    {}

    private function getBlockIndentationIndicator(string $value): string
    {}

    private function canBeDumpedAsLiteralBlock(string $value): bool
    {}
}
