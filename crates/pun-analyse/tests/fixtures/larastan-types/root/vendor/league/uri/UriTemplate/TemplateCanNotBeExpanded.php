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

use InvalidArgumentException;
use League\Uri\Contracts\UriException;

class TemplateCanNotBeExpanded extends InvalidArgumentException implements UriException
{
    public readonly array $variablesNames;

    public function __construct(string $message = '', string ...$variableNames)
    {}

    public static function dueToUnableToProcessValueListWithPrefix(string $variableName): self
    {}

    public static function dueToNestedListOfValue(string $variableName): self
    {}

    public static function dueToMissingVariables(string ...$variableNames): self
    {}
}
