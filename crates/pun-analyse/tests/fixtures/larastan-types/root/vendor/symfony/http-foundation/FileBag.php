<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\HttpFoundation;

use Symfony\Component\HttpFoundation\File\UploadedFile;

/**
 * FileBag is a container for uploaded files.
 *
 * @author Fabien Potencier <fabien@symfony.com>
 * @author Bulat Shakirzyanov <mallluhuct@gmail.com>
 */
class FileBag extends ParameterBag
{
    private const FILE_KEYS = ['error', 'full_path', 'name', 'size', 'tmp_name', 'type'];

    /**
     * @param array|UploadedFile[] $parameters An array of HTTP files
     */
    public function __construct(array $parameters = [])
    {}

    public function replace(array $files = []): void
    {}

    public function set(string $key, mixed $value): void
    {}

    public function add(array $files = []): void
    {}

    /**
     * Converts uploaded files to UploadedFile instances.
     *
     * @return UploadedFile[]|UploadedFile|null
     */
    protected function convertFileInformation(array|UploadedFile $file): array|UploadedFile|null
    {}

    /**
     * Fixes a malformed PHP $_FILES array.
     *
     * PHP has a bug that the format of the $_FILES array differs, depending on
     * whether the uploaded file fields had normal field names or array-like
     * field names ("normal" vs. "parent[child]").
     *
     * This method fixes the array to look like the "normal" $_FILES array.
     *
     * It's safe to pass an already converted array, in which case this method
     * just returns the original array unmodified.
     */
    protected function fixPhpFilesArray(array $data): array
    {}
}
