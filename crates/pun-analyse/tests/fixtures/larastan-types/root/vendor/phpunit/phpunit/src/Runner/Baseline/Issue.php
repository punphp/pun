<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Baseline;

use const FILE_IGNORE_NEW_LINES;
use function assert;
use function file;
use function is_file;
use function sha1;
use PHPUnit\Runner\FileDoesNotExistException;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class Issue
{
    /**
     * @var non-empty-string
     */
    private string $file;

    /**
     * @var positive-int
     */
    private int $line;

    /**
     * @var non-empty-string
     */
    private string $hash;

    /**
     * @var non-empty-string
     */
    private string $description;

    /**
     * @param non-empty-string  $file
     * @param positive-int      $line
     * @param ?non-empty-string $hash
     * @param non-empty-string  $description
     *
     * @throws FileDoesNotExistException
     * @throws FileDoesNotHaveLineException
     */
    public static function from(string $file, int $line, ?string $hash, string $description): self
    {}

    /**
     * @param non-empty-string $file
     * @param positive-int     $line
     * @param non-empty-string $hash
     * @param non-empty-string $description
     */
    private function __construct(string $file, int $line, string $hash, string $description)
    {}

    /**
     * @return non-empty-string
     */
    public function file(): string
    {}

    /**
     * @return positive-int
     */
    public function line(): int
    {}

    /**
     * @return non-empty-string
     */
    public function hash(): string
    {}

    /**
     * @return non-empty-string
     */
    public function description(): string
    {}

    public function equals(self $other): bool
    {}

    /**
     * @param non-empty-string $file
     * @param positive-int     $line
     *
     * @throws FileDoesNotExistException
     * @throws FileDoesNotHaveLineException
     *
     * @return non-empty-string
     */
    private static function calculateHash(string $file, int $line): string
    {}
}
