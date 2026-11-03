<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Input\File;

use Symfony\Component\Console\Exception\InvalidFileException;
use Symfony\Component\Mime\MimeTypes;

/**
 * Represents a file provided through console input.
 *
 * Inspired by HttpFoundation's UploadedFile, this class wraps a file provided
 * through console input (either pasted via terminal image protocols or typed as a path).
 *
 * @author Robin Chalas <robin.chalas@gmail.com>
 */
final class InputFile extends \SplFileInfo
{
    /** @var array<string, int|false> */
    private static array $tempFiles = [];
    private static bool $shutdownRegistered = false;

    private ?string $mimeType = null;
    private bool $isTempFile;

    public function __construct(
        string $path,
        bool $isTempFile = false,
        ?string $mimeType = null,
    ) {}

    /**
     * @throws InvalidFileException when the temporary file cannot be created
     */
    public static function fromData(string $data, ?string $format = null): self
    {}

    /**
     * @throws InvalidFileException when the file does not exist
     */
    public static function fromPath(string $path): self
    {}

    private static function normalizePath(string $path): string
    {}

    public function getMimeType(): ?string
    {}

    public function guessExtension(): ?string
    {}

    /**
     * @throws InvalidFileException when the file is invalid or the move/copy operation fails
     */
    public function move(string $directory, ?string $name = null): self
    {}

    public function cleanup(): void
    {}

    /**
     * @internal
     */
    public static function cleanupAll(): void
    {}

    public function isValid(): bool
    {}

    public function isTempFile(): bool
    {}

    /**
     * @throws InvalidFileException when the file is invalid or cannot be read
     */
    public function getContents(): string
    {}

    public function getHumanReadableSize(): string
    {}
}
