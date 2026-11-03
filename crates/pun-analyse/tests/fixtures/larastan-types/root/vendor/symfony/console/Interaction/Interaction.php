<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Interaction;

use Symfony\Component\Console\Attribute\InteractiveAttributeInterface;
use Symfony\Component\Console\Attribute\MapInput;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Output\OutputInterface;

/**
 * @internal
 */
final class Interaction
{
    public function __construct(
        private readonly object $owner,
        private readonly InteractiveAttributeInterface $attribute,
    ) {}

    /**
     * @param-immediately-invoked-callable $parameterResolver
     *
     * @param \Closure(\ReflectionFunction $function, InputInterface $input, OutputInterface $output): array $parameterResolver
     */
    public function interact(InputInterface $input, OutputInterface $output, \Closure $parameterResolver): void
    {}
}
