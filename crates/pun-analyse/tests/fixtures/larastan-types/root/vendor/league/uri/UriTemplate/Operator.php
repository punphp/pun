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

use League\Uri\Exceptions\SyntaxError;
use Stringable;

use function array_pad;
use function explode;
use function implode;
use function is_array;
use function mb_substr;
use function preg_match;
use function preg_replace_callback;
use function rawurldecode;
use function rawurlencode;
use function str_contains;

/**
 * Processing behavior according to the expression type operator.
 *
 * @internal The class exposes the internal representation of an Operator and its usage
 *
 * @link https://www.rfc-editor.org/rfc/rfc6570#section-2.2
 * @link https://tools.ietf.org/html/rfc6570#appendix-A
 */
enum Operator: string
{
    private const REGEXP_CHARS_INVALID = '/[\x00-\x1f\x7f]/';
    private const REGEXP_CHARS_NOT_UNRESERVED = '/[^A-Za-z0-9_\-.~]/';
    private const REGEXP_CHARS_ENCODED = ',%[A-Fa-f0-9]{2},';
    private const REGEXP_CHARS_TO_ENCODE = '/[^A-Za-z0-9_\-.~!$&\'()*+,;=%:@\/?]+|%(?![A-Fa-f0-9]{2})/';

    /**
     * Expression regular expression pattern.
     *
     * @link https://tools.ietf.org/html/rfc6570#section-2.2
     */
    private const REGEXP_EXPRESSION = '/^\{(?:(?<operator>[\.\/;\?&\=,\!@\|\+#])?(?<variables>[^\}]*))\}$/';

    /**
     * Reserved Operator characters.
     *
     * @link https://tools.ietf.org/html/rfc6570#section-2.2
     */
    private const RESERVED_OPERATOR = '=,!@|';

    case None = '';
    case ReservedChars = '+';
    case Label = '.';
    case Path = '/';
    case PathParam = ';';
    case Query = '?';
    case QueryPair = '&';
    case Fragment = '#';

    public function nextDelimiter(): ?string
    {}

    public function first(): string
    {}

    public function separator(): string
    {}

    public function supportsListValue(): bool
    {}

    public function isNamed(): bool
    {}

    public function allowEmpty(): bool
    {}

    /**
     * Removes percent encoding on reserved characters (used with + and # modifiers).
     */
    public function encode(string $value): string
    {}

    /**
     * Decodes a URI template value.
     *
     * @throws SyntaxError if the value to decode contains invalid characters.
     */
    public function decode(string $value): string
    {}

    /**
     * @throws SyntaxError if the expression is invalid
     * @throws SyntaxError if the operator used in the expression is invalid
     * @throws SyntaxError if the contained variable specifiers are invalid
     *
     * @return array{operator:Operator, variables:string}
     */
    public static function parseExpression(Stringable|string $expression): array
    {}

    /**
     * Replaces an expression with the given variables.
     *
     * @throws TemplateCanNotBeExpanded if the variables is an array and a ":" modifier needs to be applied
     * @throws TemplateCanNotBeExpanded if the variables contains nested array values
     */
    public function expand(VarSpecifier $varSpecifier, VariableBag $variables): string
    {}

    /**
     * @param string|array<string> $value
     *
     * @return array{0:string, 1:bool}
     */
    private function inject(array|string $value, VarSpecifier $varSpec): array
    {}

    /**
     * Expands an expression using a list of values.
     *
     * @param array<string> $value
     *
     * @throws TemplateCanNotBeExpanded if the variables is an array and a ":" modifier needs to be applied
     *
     * @return array{0:string, 1:bool}
     */
    private function replaceList(array $value, VarSpecifier $varSpec): array
    {}

    /**
     * Extracts a variable from an operator value.
     *
     * A null value represents an absent variable. Exploded variables are delegated
     * to list extraction, while named non-exploded variables must contain the
     * expected variable name. Path and fragment operators may represent non-exploded
     * values as key/value pairs. Extracted values are percent-decoded before being
     * returned.
     *
     * @throws VariableCanNotBeExtracted If the value cannot be extracted according
     *                                   to the variable specifier.
     */
    public function extract(VarSpecifier $varSpecifier, string|null $value): ExtractionResult
    {}

    /**
     * @throws VariableCanNotBeExtracted
     */
    private function extractNamedValue(VarSpecifier $varSpecifier, string $value): ExtractedValue
    {}

    private function extractFragmentValue(VarSpecifier $varSpecifier, string $value): ExtractedValue
    {}
}
