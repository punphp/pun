<?php declare(strict_types=1);
/*
 * This file is part of sebastian/diff.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Diff;

final class Line
{
    public const int ADDED     = 1;
    public const int REMOVED   = 2;
    public const int UNCHANGED = 3;
    private int $type;
    private string $content;

    public function __construct(int $type = self::UNCHANGED, string $content = '')
    {}

    public function content(): string
    {}

    public function type(): int
    {}

    public function isAdded(): bool
    {}

    public function isRemoved(): bool
    {}

    public function isUnchanged(): bool
    {}
}
