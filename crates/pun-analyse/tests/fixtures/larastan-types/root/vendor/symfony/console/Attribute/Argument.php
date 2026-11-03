<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Attribute;

use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Completion\CompletionInput;
use Symfony\Component\Console\Completion\Suggestion;
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\String\UnicodeString;

#[\Attribute(\Attribute::TARGET_PARAMETER | \Attribute::TARGET_PROPERTY)]
class Argument
{
    public mixed $default = null;
    public array|\Closure $suggestedValues;

    /**
     * @internal
     *
     * @var string|class-string<\BackedEnum>
     */
    public string $typeName = '';
    private ?int $mode = null;
    private ?InteractiveAttributeInterface $interactiveAttribute = null;

    /**
     * Represents a console command <argument> definition.
     *
     * If unset, the `name` value will be inferred from the parameter definition.
     *
     * @param array<string|Suggestion>|callable(CompletionInput):list<string|Suggestion> $suggestedValues The values used for input completion
     */
    public function __construct(
        public string $description = '',
        public string $name = '',
        array|callable $suggestedValues = [],
    ) {}

    /**
     * @internal
     */
    public static function tryFrom(\ReflectionParameter|\ReflectionProperty $member): ?self
    {}

    /**
     * @internal
     */
    public function toInputArgument(): InputArgument
    {}

    /**
     * @internal
     */
    public function getInteractiveAttribute(): ?InteractiveAttributeInterface
    {}

    /**
     * @internal
     */
    public function isRequired(): bool
    {}
}
