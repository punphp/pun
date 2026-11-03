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
 * @author Yewhen Khoptynskyi <khoptynskyi@gmail.com>
 */
class TableCellStyle
{
    public const DEFAULT_ALIGN = 'left';

    private const TAG_OPTIONS = [
        'fg',
        'bg',
        'options',
    ];

    private const ALIGN_MAP = [
        'left' => \STR_PAD_RIGHT,
        'center' => \STR_PAD_BOTH,
        'right' => \STR_PAD_LEFT,
    ];

    private array $options = [
        'fg' => 'default',
        'bg' => 'default',
        'options' => null,
        'align' => self::DEFAULT_ALIGN,
        'cellFormat' => null,
    ];

    public function __construct(array $options = [])
    {}

    public function getOptions(): array
    {}

    /**
     * Gets options we need for tag for example fg, bg.
     *
     * @return string[]
     */
    public function getTagOptions(): array
    {}

    public function getPadByAlign(): int
    {}

    public function getCellFormat(): ?string
    {}
}
