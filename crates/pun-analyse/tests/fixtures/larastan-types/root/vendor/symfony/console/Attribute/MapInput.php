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
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Interaction\Interaction;

/**
 * Maps a command input into an object (DTO).
 */
#[\Attribute(\Attribute::TARGET_PARAMETER | \Attribute::TARGET_PROPERTY)]
final class MapInput
{
    /**
     * @var array<string, Argument|Option|self>
     */
    private array $definition = [];

    private \ReflectionClass $class;

    /**
     * @var list<Interact>
     */
    private array $interactiveAttributes = [];

    /**
     * @param string[]|null $validationGroups
     */
    public function __construct(
        public readonly ?array $validationGroups = null,
    ) {}

    /**
     * @internal
     */
    public static function tryFrom(\ReflectionParameter|\ReflectionProperty $member): ?self
    {}

    /**
     * @internal
     */
    public function setValue(InputInterface $input, object $object): void
    {}

    /**
     * @return iterable<Argument>
     */
    public function getArguments(): iterable
    { yield; }

    /**
     * @return iterable<Option>
     */
    public function getOptions(): iterable
    { yield; }

    /**
     * @internal
     *
     * @return \ReflectionClass<object>
     */
    public function getClass(): \ReflectionClass
    {}

    /**
     * @internal
     *
     * @return array<string, Argument|Option|self>
     */
    public function getDefinition(): array
    {}

    /**
     * Creates a populated instance of the DTO from command input.
     *
     * @internal
     */
    public function createInstance(InputInterface $input): object
    {}

    private function resolveValue(string $typeName, mixed $value, mixed $default): mixed
    {}

    /**
     * @internal
     *
     * @return iterable<Interaction>
     */
    public function getPropertyInteractions(): iterable
    { yield; }

    /**
     * @internal
     *
     * @return iterable<Interaction>
     */
    public function getMethodInteractions(): iterable
    { yield; }
}
