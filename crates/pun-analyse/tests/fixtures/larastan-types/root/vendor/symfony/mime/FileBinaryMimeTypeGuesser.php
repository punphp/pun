<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Mime;

use Symfony\Component\Mime\Exception\InvalidArgumentException;
use Symfony\Component\Mime\Exception\LogicException;

/**
 * Guesses the MIME type with the binary "file" (only available on *nix).
 *
 * @author Bernhard Schussek <bschussek@gmail.com>
 */
class FileBinaryMimeTypeGuesser implements MimeTypeGuesserInterface
{
    /**
     * The $cmd pattern must contain a "%s" string that will be replaced
     * with the file name to guess.
     *
     * The command output must start with the MIME type of the file.
     *
     * @param string $cmd The command to run to get the MIME type of a file
     */
    public function __construct(
        private string $cmd = 'file -b --mime -- %s 2>/dev/null',
    ) {}

    public function isGuesserSupported(): bool
    {}

    public function guessMimeType(string $path): ?string
    {}
}
