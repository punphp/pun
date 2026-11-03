<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Formatter;

use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Contracts\Service\ResetInterface;

/**
 * @author Jean-François Simon <contact@jfsimon.fr>
 */
class OutputFormatterStyleStack implements ResetInterface
{
    /**
     * @var OutputFormatterStyleInterface[]
     */
    private array $styles = [];

    private OutputFormatterStyleInterface $emptyStyle;

    public function __construct(?OutputFormatterStyleInterface $emptyStyle = null)
    {}

    /**
     * Resets stack (ie. empty internal arrays).
     */
    public function reset(): void
    {}

    /**
     * Pushes a style in the stack.
     */
    public function push(OutputFormatterStyleInterface $style): void
    {}

    /**
     * Pops a style from the stack.
     *
     * @throws InvalidArgumentException When style tags incorrectly nested
     */
    public function pop(?OutputFormatterStyleInterface $style = null): OutputFormatterStyleInterface
    {}

    /**
     * Computes current style with stacks top codes.
     */
    public function getCurrent(): OutputFormatterStyleInterface
    {}

    /**
     * @return $this
     */
    public function setEmptyStyle(OutputFormatterStyleInterface $emptyStyle): static
    {}

    public function getEmptyStyle(): OutputFormatterStyleInterface
    {}
}
