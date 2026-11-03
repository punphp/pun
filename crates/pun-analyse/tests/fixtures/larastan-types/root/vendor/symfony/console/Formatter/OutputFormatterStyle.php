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

use Symfony\Component\Console\Color;

/**
 * Formatter style class for defining styles.
 *
 * @author Konstantin Kudryashov <ever.zet@gmail.com>
 */
class OutputFormatterStyle implements OutputFormatterStyleInterface
{
    private Color $color;
    private string $foreground;
    private string $background;
    private array $options;
    private ?string $href = null;
    private bool $handlesHrefGracefully;

    /**
     * Initializes output formatter style.
     *
     * @param string|null $foreground The style foreground color name
     * @param string|null $background The style background color name
     */
    public function __construct(?string $foreground = null, ?string $background = null, array $options = [])
    {}

    public function setForeground(?string $color): void
    {}

    public function setBackground(?string $color): void
    {}

    public function setHref(string $url): void
    {}

    public function setOption(string $option): void
    {}

    public function unsetOption(string $option): void
    {}

    public function setOptions(array $options): void
    {}

    public function apply(string $text): string
    {}
}
