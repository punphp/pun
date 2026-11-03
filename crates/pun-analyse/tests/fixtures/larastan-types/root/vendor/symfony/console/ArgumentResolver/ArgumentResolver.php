<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\ArgumentResolver;

use Psr\Container\ContainerInterface;
use Symfony\Component\Console\Application;
use Symfony\Component\Console\ArgumentResolver\Exception\NearMissValueResolverException;
use Symfony\Component\Console\ArgumentResolver\Exception\ResolverNotFoundException;
use Symfony\Component\Console\ArgumentResolver\ValueResolver as Resolver;
use Symfony\Component\Console\ArgumentResolver\ValueResolver\ValueResolverInterface;
use Symfony\Component\Console\Attribute\Argument;
use Symfony\Component\Console\Attribute\Option;
use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Attribute\ValueResolver;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Cursor;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Input\RawInputInterface;
use Symfony\Component\Console\Output\OutputInterface;
use Symfony\Component\Console\Style\SymfonyStyle;
use Symfony\Contracts\Service\ServiceProviderInterface;

/**
 * Resolves the arguments passed to a console command.
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
final class ArgumentResolver implements ArgumentResolverInterface
{
    private const CORE_UTILITY_TYPES = [
        InputInterface::class,
        RawInputInterface::class,
        OutputInterface::class,
        SymfonyStyle::class,
        Cursor::class,
        Application::class,
        Command::class,
    ];

    /**
     * @param iterable<mixed, ValueResolverInterface> $argumentValueResolvers
     */
    public function __construct(
        private iterable $argumentValueResolvers = [],
        private ?ContainerInterface $namedResolvers = null,
    ) {}

    public function getArguments(InputInterface $input, callable $command, ?\ReflectionFunctionAbstract $reflector = null): array
    {}

    /**
     * @return iterable<int, ValueResolverInterface>
     */
    public static function getDefaultArgumentValueResolvers(): iterable
    {}
}
