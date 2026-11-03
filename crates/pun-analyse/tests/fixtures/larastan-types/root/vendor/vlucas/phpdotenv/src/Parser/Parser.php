<?php

declare(strict_types=1);

namespace Dotenv\Parser;

use Dotenv\Exception\InvalidFileException;
use Dotenv\Util\Regex;
use GrahamCampbell\ResultType\Error;
use GrahamCampbell\ResultType\Success;

final class Parser implements ParserInterface
{
    /**
     * Parse content into an entry array.
     *
     * @param string $content
     *
     * @throws \Dotenv\Exception\InvalidFileException
     *
     * @return \Dotenv\Parser\Entry[]
     */
    public function parse(string $content)
    {}

    /**
     * Convert the raw entries into proper entries.
     *
     * @param string[] $entries
     *
     * @return \GrahamCampbell\ResultType\Result<\Dotenv\Parser\Entry[], string>
     */
    private static function process(array $entries)
    {}
}
