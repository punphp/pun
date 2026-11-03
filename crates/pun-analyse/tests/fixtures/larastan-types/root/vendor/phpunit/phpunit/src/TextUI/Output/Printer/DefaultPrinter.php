<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Output;

use function assert;
use function count;
use function dirname;
use function explode;
use function fclose;
use function fopen;
use function fsockopen;
use function fwrite;
use function str_replace;
use function str_starts_with;
use PHPUnit\Runner\DirectoryDoesNotExistException;
use PHPUnit\TextUI\CannotOpenSocketException;
use PHPUnit\TextUI\InvalidSocketException;
use PHPUnit\Util\Filesystem;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class DefaultPrinter implements Printer
{
    /**
     * @var closed-resource|resource
     */
    private $stream;
    private readonly bool $isPhpStream;
    private bool $isOpen;

    /**
     * @throws CannotOpenSocketException
     * @throws DirectoryDoesNotExistException
     * @throws InvalidSocketException
     */
    public static function from(string $out): self
    {}

    /**
     * @throws CannotOpenSocketException
     * @throws DirectoryDoesNotExistException
     * @throws InvalidSocketException
     */
    public static function standardOutput(): self
    {}

    /**
     * @throws CannotOpenSocketException
     * @throws DirectoryDoesNotExistException
     * @throws InvalidSocketException
     */
    public static function standardError(): self
    {}

    /**
     * @throws CannotOpenSocketException
     * @throws DirectoryDoesNotExistException
     * @throws InvalidSocketException
     */
    private function __construct(string $out)
    {}

    public function print(string $buffer): void
    {}

    public function flush(): void
    {}
}
