<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Helper;

use Symfony\Component\Console\Exception\InvalidArgumentException;

/**
 * @author Abdellatif Ait boudad <a.aitboudad@gmail.com>
 */
class TableCell
{
    private array $options = [
        'rowspan' => 1,
        'colspan' => 1,
        'style' => null,
    ];

    public function __construct(
        private string $value = '',
        array $options = [],
    ) {}

    /**
     * Returns the cell value.
     */
    public function __toString(): string
    {}

    /**
     * Gets number of colspan.
     */
    public function getColspan(): int
    {}

    /**
     * Gets number of rowspan.
     */
    public function getRowspan(): int
    {}

    public function getStyle(): ?TableCellStyle
    {}
}
