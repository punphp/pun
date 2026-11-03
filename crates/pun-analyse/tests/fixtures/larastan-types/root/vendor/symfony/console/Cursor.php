<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console;

use Symfony\Component\Console\Output\OutputInterface;

/**
 * @author Pierre du Plessis <pdples@gmail.com>
 */
final class Cursor
{
    /** @var resource */
    private $input;

    /**
     * @param resource|null $input
     */
    public function __construct(
        private OutputInterface $output,
        $input = null,
    ) {}

    /**
     * @return $this
     */
    public function moveUp(int $lines = 1): static
    {}

    /**
     * @return $this
     */
    public function moveDown(int $lines = 1): static
    {}

    /**
     * @return $this
     */
    public function moveRight(int $columns = 1): static
    {}

    /**
     * @return $this
     */
    public function moveLeft(int $columns = 1): static
    {}

    /**
     * @return $this
     */
    public function moveToColumn(int $column): static
    {}

    /**
     * @return $this
     */
    public function moveToPosition(int $column, int $row): static
    {}

    /**
     * @return $this
     */
    public function savePosition(): static
    {}

    /**
     * @return $this
     */
    public function restorePosition(): static
    {}

    /**
     * @return $this
     */
    public function hide(): static
    {}

    /**
     * @return $this
     */
    public function show(): static
    {}

    /**
     * Clears all the output from the current line.
     *
     * @return $this
     */
    public function clearLine(): static
    {}

    /**
     * Clears all the output from the current line after the current position.
     */
    public function clearLineAfter(): self
    {}

    /**
     * Clears all the output from the cursors' current position to the end of the screen.
     *
     * @return $this
     */
    public function clearOutput(): static
    {}

    /**
     * Clears the entire screen.
     *
     * @return $this
     */
    public function clearScreen(): static
    {}

    /**
     * Returns the current cursor position as x,y coordinates.
     */
    public function getCurrentPosition(): array
    {}
}
