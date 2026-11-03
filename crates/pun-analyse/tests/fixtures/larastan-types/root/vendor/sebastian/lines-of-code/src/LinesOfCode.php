<?php declare(strict_types=1);
/*
 * This file is part of sebastian/lines-of-code.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\LinesOfCode;

/**
 * @immutable
 */
final readonly class LinesOfCode
{
    /**
     * @var positive-int
     */
    private int $linesOfCode;

    /**
     * @var non-negative-int
     */
    private int $commentLinesOfCode;

    /**
     * @var non-negative-int
     */
    private int $nonCommentLinesOfCode;

    /**
     * @var non-negative-int
     */
    private int $logicalLinesOfCode;

    /**
     * @param positive-int     $linesOfCode
     * @param non-negative-int $commentLinesOfCode
     * @param non-negative-int $nonCommentLinesOfCode
     * @param non-negative-int $logicalLinesOfCode
     *
     * @throws IllogicalValuesException
     */
    public function __construct(int $linesOfCode, int $commentLinesOfCode, int $nonCommentLinesOfCode, int $logicalLinesOfCode)
    {}

    /**
     * @return positive-int
     */
    public function linesOfCode(): int
    {}

    /**
     * @return non-negative-int
     */
    public function commentLinesOfCode(): int
    {}

    /**
     * @return non-negative-int
     */
    public function nonCommentLinesOfCode(): int
    {}

    /**
     * @return non-negative-int
     */
    public function logicalLinesOfCode(): int
    {}

    public function plus(self $other): self
    {}
}
