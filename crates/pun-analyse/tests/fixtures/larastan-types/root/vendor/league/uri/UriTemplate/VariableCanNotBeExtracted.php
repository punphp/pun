<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri\UriTemplate;

use Exception;
use League\Uri\Contracts\UriException;
use Throwable;
use TypeError;

use function array_values;
use function get_debug_type;
use function implode;
use function is_string;

class VariableCanNotBeExtracted extends Exception implements UriException
{
    /** @var list<string> */
    public readonly array $names;
    /** @var list<string> */
    public readonly array $missingNames;
    /** @var list<ExtractionErrorReason> */
    public readonly array $reasons;

    protected function __construct(
        string $message,
        array $reasons,
        array $names = [],
        array $missingNames = [],
        ?Throwable $previous = null,
    ) {}

    /**
     * @return list<ExtractionErrorReason>
     */
    private static function uniqueReasons(iterable $reasons): array
    {}

    /**
     * @return list<string>
     */
    private static function uniqueNames(iterable $names, string $message): array
    {}

    public static function dueTo(string $message, ExtractionErrorReason ...$reasons): self
    {}

    public static function dueToMissingVariables(string $input, Template $template, ExtractionResult|iterable $missingNames): self
    {}

    public static function dueToExtractionFailure(string $input, Template $template, self $previous): self
    {}

    public static function dueToSuitableCandidateNotFound(string $input, array $reasons, array $missingNames = []): self
    {}
}
