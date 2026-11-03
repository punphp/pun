<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\ArgumentResolver\ValueResolver;

use Symfony\Component\Console\Attribute\Argument;
use Symfony\Component\Console\Attribute\MapInput;
use Symfony\Component\Console\Attribute\Option;
use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Exception\InputValidationFailedException;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Validator\Validator\ValidatorInterface;

/**
 * Resolves the value of a input argument/option to an object holding the #[MapInput] attribute.
 *
 * @author Yonel Ceruto <open@yceruto.dev>
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
final class MapInputValueResolver implements ValueResolverInterface
{
    public function __construct(
        private readonly ValueResolverInterface $builtinTypeResolver,
        private readonly ValueResolverInterface $backedEnumResolver,
        private readonly ValueResolverInterface $dateTimeResolver,
        private readonly ?ValidatorInterface $validator = null,
    ) {}

    public function resolve(string $argumentName, InputInterface $input, ReflectionMember $member): iterable
    {}

    private function resolveMapInput(MapInput $mapInput, InputInterface $input): object
    {}

    /**
     * @return array<string, string>
     */
    private function buildPropertyToInputMap(MapInput $mapInput, string $prefix = ''): array
    {}

    private function resolveArgumentSpec(Argument $argument, \ReflectionProperty $property, InputInterface $input): mixed
    {}

    private function resolveOptionSpec(Option $option, \ReflectionProperty $property, InputInterface $input): mixed
    {}
}
