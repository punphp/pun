<?php

declare(strict_types=1);

namespace League\Flysystem\Local;

use const DIRECTORY_SEPARATOR;
use const LOCK_EX;
use DirectoryIterator;
use FilesystemIterator;
use Generator;
use League\Flysystem\ChecksumProvider;
use League\Flysystem\Config;
use League\Flysystem\DirectoryAttributes;
use League\Flysystem\FileAttributes;
use League\Flysystem\FilesystemAdapter;
use League\Flysystem\PathPrefixer;
use League\Flysystem\SymbolicLinkEncountered;
use League\Flysystem\UnableToCopyFile;
use League\Flysystem\UnableToCreateDirectory;
use League\Flysystem\UnableToDeleteDirectory;
use League\Flysystem\UnableToDeleteFile;
use League\Flysystem\UnableToMoveFile;
use League\Flysystem\UnableToProvideChecksum;
use League\Flysystem\UnableToReadFile;
use League\Flysystem\UnableToRetrieveMetadata;
use League\Flysystem\UnableToSetVisibility;
use League\Flysystem\UnableToWriteFile;
use League\Flysystem\UnixVisibility\PortableVisibilityConverter;
use League\Flysystem\UnixVisibility\VisibilityConverter;
use League\MimeTypeDetection\FinfoMimeTypeDetector;
use League\MimeTypeDetection\MimeTypeDetector;
use RecursiveDirectoryIterator;
use RecursiveIteratorIterator;
use SplFileInfo;
use Throwable;
use function chmod;
use function clearstatcache;
use function dirname;
use function error_clear_last;
use function error_get_last;
use function file_put_contents;
use function hash_file;
use function is_dir;
use function is_file;
use function mkdir;
use function rename;

class LocalFilesystemAdapter implements FilesystemAdapter, ChecksumProvider
{
    /**
     * @var int
     */
    public const SKIP_LINKS = 0001;

    /**
     * @var int
     */
    public const DISALLOW_LINKS = 0002;

    private PathPrefixer $prefixer;
    private VisibilityConverter $visibility;
    private MimeTypeDetector $mimeTypeDetector;
    private string $rootLocation;

    /**
     * @var bool
     */
    private $rootLocationIsSetup = false;

    public function __construct(
        string $location,
        ?VisibilityConverter $visibility = null,
        private int $writeFlags = LOCK_EX,
        private int $linkHandling = self::DISALLOW_LINKS,
        ?MimeTypeDetector $mimeTypeDetector = null,
        bool $lazyRootCreation = false,
        bool $useInconclusiveMimeTypeFallback = false,
    ) {}

    private function ensureRootDirectoryExists(): void
    {}

    public function write(string $path, string $contents, Config $config): void
    {}

    public function writeStream(string $path, $contents, Config $config): void
    {}

    /**
     * @param resource|string $contents
     */
    private function writeToFile(string $path, $contents, Config $config): void
    {}

    public function delete(string $path): void
    {}

    public function deleteDirectory(string $prefix): void
    {}

    private function listDirectoryRecursively(
        string $path,
        int $mode = RecursiveIteratorIterator::SELF_FIRST
    ): Generator { yield; }

    protected function deleteFileInfoObject(SplFileInfo $file): bool
    {}

    public function listContents(string $path, bool $deep): iterable
    { yield; }

    public function move(string $source, string $destination, Config $config): void
    {}

    public function copy(string $source, string $destination, Config $config): void
    {}

    public function read(string $path): string
    {}

    public function readStream(string $path)
    {}

    protected function ensureDirectoryExists(string $dirname, int $visibility): void
    {}

    public function fileExists(string $location): bool
    {}

    public function directoryExists(string $location): bool
    {}

    public function createDirectory(string $path, Config $config): void
    {}

    public function setVisibility(string $path, string $visibility): void
    {}

    public function visibility(string $path): FileAttributes
    {}

    private function resolveDirectoryVisibility(?string $visibility): int
    {}

    public function mimeType(string $path): FileAttributes
    {}

    public function lastModified(string $path): FileAttributes
    {}

    public function fileSize(string $path): FileAttributes
    {}

    public function checksum(string $path, Config $config): string
    {}

    private function listDirectory(string $location): Generator
    { yield; }

    private function setPermissions(string $location, int $visibility): void
    {}
}
