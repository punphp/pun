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
final class NullOutputFormatter implements OutputFormatterInterface
{
    private NullOutputFormatterStyle $style;

    public function format(?string $message): ?string
    {}

    public function getStyle(string $name): OutputFormatterStyleInterface
    {}

    public function hasStyle(string $name): bool
    {}

    public function isDecorated(): bool
    {}

    public function setDecorated(bool $decorated): void
    {}

    public function setStyle(string $name, OutputFormatterStyleInterface $style): void
    {}
}
