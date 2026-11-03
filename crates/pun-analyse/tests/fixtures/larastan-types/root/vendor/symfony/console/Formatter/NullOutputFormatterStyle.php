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

/**
 * @author Tien Xuan Vo <tien.xuan.vo@gmail.com>
 */
final class NullOutputFormatterStyle implements OutputFormatterStyleInterface
{
    public function apply(string $text): string
    {}

    public function setBackground(?string $color): void
    {}

    public function setForeground(?string $color): void
    {}

    public function setOption(string $option): void
    {}

    public function setOptions(array $options): void
    {}

    public function unsetOption(string $option): void
    {}
}
