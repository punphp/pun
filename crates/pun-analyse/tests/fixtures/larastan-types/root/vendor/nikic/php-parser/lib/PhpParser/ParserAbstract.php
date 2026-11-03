<?php declare(strict_types=1);

namespace PhpParser;

/*
 * This parser is based on a skeleton written by Moriyoshi Koizumi, which in
 * turn is based on work by Masato Bito.
 */

use PhpParser\Node\Arg;
use PhpParser\Node\Expr;
use PhpParser\Node\Expr\Array_;
use PhpParser\Node\Expr\Cast\Double;
use PhpParser\Node\Identifier;
use PhpParser\Node\InterpolatedStringPart;
use PhpParser\Node\Name;
use PhpParser\Node\Param;
use PhpParser\Node\PropertyHook;
use PhpParser\Node\Scalar\InterpolatedString;
use PhpParser\Node\Scalar\Int_;
use PhpParser\Node\Scalar\String_;
use PhpParser\Node\Stmt;
use PhpParser\Node\Stmt\Class_;
use PhpParser\Node\Stmt\ClassConst;
use PhpParser\Node\Stmt\ClassMethod;
use PhpParser\Node\Stmt\Const_;
use PhpParser\Node\Stmt\Else_;
use PhpParser\Node\Stmt\ElseIf_;
use PhpParser\Node\Stmt\Enum_;
use PhpParser\Node\Stmt\Interface_;
use PhpParser\Node\Stmt\Namespace_;
use PhpParser\Node\Stmt\Nop;
use PhpParser\Node\Stmt\Property;
use PhpParser\Node\Stmt\TryCatch;
use PhpParser\Node\UseItem;
use PhpParser\Node\VarLikeIdentifier;
use PhpParser\NodeVisitor\CommentAnnotatingVisitor;

abstract class ParserAbstract implements Parser {
    private const SYMBOL_NONE = -1;

    /** @var Lexer Lexer that is used when parsing */
    protected Lexer $lexer;
    /** @var PhpVersion PHP version to target on a best-effort basis */
    protected PhpVersion $phpVersion;

    /*
     * The following members will be filled with generated parsing data:
     */

    /** @var int Size of $tokenToSymbol map */
    protected int $tokenToSymbolMapSize;
    /** @var int Size of $action table */
    protected int $actionTableSize;
    /** @var int Size of $goto table */
    protected int $gotoTableSize;

    /** @var int Symbol number signifying an invalid token */
    protected int $invalidSymbol;
    /** @var int Symbol number of error recovery token */
    protected int $errorSymbol;
    /** @var int Action number signifying default action */
    protected int $defaultAction;
    /** @var int Rule number signifying that an unexpected token was encountered */
    protected int $unexpectedTokenRule;

    protected int $YY2TBLSTATE;
    /** @var int Number of non-leaf states */
    protected int $numNonLeafStates;

    /** @var int[] Map of PHP token IDs to internal symbols */
    protected array $phpTokenToSymbol;
    /** @var array<int, bool> Map of PHP token IDs to drop */
    protected array $dropTokens;
    /** @var int[] Map of external symbols (static::T_*) to internal symbols */
    protected array $tokenToSymbol;
    /** @var string[] Map of symbols to their names */
    protected array $symbolToName;
    /** @var array<int, string> Names of the production rules (only necessary for debugging) */
    protected array $productions;

    /** @var int[] Map of states to a displacement into the $action table. The corresponding action for this
     *             state/symbol pair is $action[$actionBase[$state] + $symbol]. If $actionBase[$state] is 0, the
     *             action is defaulted, i.e. $actionDefault[$state] should be used instead. */
    protected array $actionBase;
    /** @var int[] Table of actions. Indexed according to $actionBase comment. */
    protected array $action;
    /** @var int[] Table indexed analogously to $action. If $actionCheck[$actionBase[$state] + $symbol] != $symbol
     *             then the action is defaulted, i.e. $actionDefault[$state] should be used instead. */
    protected array $actionCheck;
    /** @var int[] Map of states to their default action */
    protected array $actionDefault;
    /** @var callable[] Semantic action callbacks */
    protected array $reduceCallbacks;

    /** @var int[] Map of non-terminals to a displacement into the $goto table. The corresponding goto state for this
     *             non-terminal/state pair is $goto[$gotoBase[$nonTerminal] + $state] (unless defaulted) */
    protected array $gotoBase;
    /** @var int[] Table of states to goto after reduction. Indexed according to $gotoBase comment. */
    protected array $goto;
    /** @var int[] Table indexed analogously to $goto. If $gotoCheck[$gotoBase[$nonTerminal] + $state] != $nonTerminal
     *             then the goto state is defaulted, i.e. $gotoDefault[$nonTerminal] should be used. */
    protected array $gotoCheck;
    /** @var int[] Map of non-terminals to the default state to goto after their reduction */
    protected array $gotoDefault;

    /** @var int[] Map of rules to the non-terminal on their left-hand side, i.e. the non-terminal to use for
     *             determining the state to goto after reduction. */
    protected array $ruleToNonTerminal;
    /** @var int[] Map of rules to the length of their right-hand side, which is the number of elements that have to
     *             be popped from the stack(s) on reduction. */
    protected array $ruleToLength;

    /*
     * The following members are part of the parser state:
     */

    /** @var mixed Temporary value containing the result of last semantic action (reduction) */
    protected $semValue;
    /** @var mixed[] Semantic value stack (contains values of tokens and semantic action results) */
    protected array $semStack;
    /** @var int[] Token start position stack */
    protected array $tokenStartStack;
    /** @var int[] Token end position stack */
    protected array $tokenEndStack;

    /** @var ErrorHandler Error handler */
    protected ErrorHandler $errorHandler;
    /** @var int Error state, used to avoid error floods */
    protected int $errorState;

    /** @var \SplObjectStorage<Array_, null>|null Array nodes created during parsing, for postprocessing of empty elements. */
    protected ?\SplObjectStorage $createdArrays;

    /** @var \SplObjectStorage<Expr\ArrowFunction, null>|null
     *       Arrow functions that are wrapped in parentheses, to enforce the pipe operator parentheses requirements.
     */
    protected ?\SplObjectStorage $parenthesizedArrowFunctions;

    /** @var Token[] Tokens for the current parse */
    protected array $tokens;
    /** @var int Current position in token array */
    protected int $tokenPos;

    /**
     * Initialize $reduceCallbacks map.
     */
    abstract protected function initReduceCallbacks(): void;

    /**
     * Creates a parser instance.
     *
     * Options:
     *  * phpVersion: ?PhpVersion,
     *
     * @param Lexer $lexer A lexer
     * @param PhpVersion $phpVersion PHP version to target, defaults to latest supported. This
     *                               option is best-effort: Even if specified, parsing will generally assume the latest
     *                               supported version and only adjust behavior in minor ways, for example by omitting
     *                               errors in older versions and interpreting type hints as a name or identifier depending
     *                               on version.
     */
    public function __construct(Lexer $lexer, ?PhpVersion $phpVersion = null) {}

    /**
     * Parses PHP code into a node tree.
     *
     * If a non-throwing error handler is used, the parser will continue parsing after an error
     * occurred and attempt to build a partial AST.
     *
     * @param string $code The source code to parse
     * @param ErrorHandler|null $errorHandler Error handler to use for lexer/parser errors, defaults
     *                                        to ErrorHandler\Throwing.
     *
     * @return Node\Stmt[]|null Array of statements (or null non-throwing error handler is used and
     *                          the parser was unable to recover from an error).
     */
    public function parse(string $code, ?ErrorHandler $errorHandler = null): ?array {}

    public function getTokens(): array {}

    /** @return Stmt[]|null */
    protected function doParse(): ?array {}

    protected function emitError(Error $error): void {}

    /**
     * Format error message including expected tokens.
     *
     * @param int $symbol Unexpected symbol
     * @param int $state State at time of error
     *
     * @return string Formatted error message
     */
    protected function getErrorMessage(int $symbol, int $state): string {}

    /**
     * Get limited number of expected tokens in given state.
     *
     * @param int $state State
     *
     * @return string[] Expected tokens. If too many, an empty array is returned.
     */
    protected function getExpectedTokens(int $state): array {}

    /**
     * Get attributes for a node with the given start and end token positions.
     *
     * @param int $tokenStartPos Token position the node starts at
     * @param int $tokenEndPos Token position the node ends at
     * @return array<string, mixed> Attributes
     */
    protected function getAttributes(int $tokenStartPos, int $tokenEndPos): array {}

    /**
     * Get attributes for a single token at the given token position.
     *
     * @return array<string, mixed> Attributes
     */
    protected function getAttributesForToken(int $tokenPos): array {}

    /*
     * Tracing functions used for debugging the parser.
     */

    /*
    protected function traceNewState($state, $symbol): void {
        echo '% State ' . $state
            . ', Lookahead ' . ($symbol == self::SYMBOL_NONE ? '--none--' : $this->symbolToName[$symbol]) . "\n";
    }

    protected function traceRead($symbol): void {
        echo '% Reading ' . $this->symbolToName[$symbol] . "\n";
    }

    protected function traceShift($symbol): void {
        echo '% Shift ' . $this->symbolToName[$symbol] . "\n";
    }

    protected function traceAccept(): void {
        echo "% Accepted.\n";
    }

    protected function traceReduce($n): void {
        echo '% Reduce by (' . $n . ') ' . $this->productions[$n] . "\n";
    }

    protected function tracePop($state): void {
        echo '% Recovering, uncovered state ' . $state . "\n";
    }

    protected function traceDiscard($symbol): void {
        echo '% Discard ' . $this->symbolToName[$symbol] . "\n";
    }
    */

    /*
     * Helper functions invoked by semantic actions
     */

    /**
     * Moves statements of semicolon-style namespaces into $ns->stmts and checks various error conditions.
     *
     * @param Node\Stmt[] $stmts
     * @return Node\Stmt[]
     */
    protected function handleNamespaces(array $stmts): array {}

    private function fixupNamespaceAttributes(Node\Stmt\Namespace_ $stmt): void {}

    /** @return array<string, mixed> */
    private function getNamespaceErrorAttributes(Namespace_ $node): array {}

    /**
     * Determine namespacing style (semicolon or brace)
     *
     * @param Node[] $stmts Top-level statements.
     *
     * @return null|string One of "semicolon", "brace" or null (no namespaces)
     */
    private function getNamespacingStyle(array $stmts): ?string {}

    /** @return Name|Identifier */
    protected function handleBuiltinTypes(Name $name) {}

    /**
     * Get combined start and end attributes at a stack location
     *
     * @param int $stackPos Stack location
     *
     * @return array<string, mixed> Combined start and end attributes
     */
    protected function getAttributesAt(int $stackPos): array {}

    protected function getFloatCastKind(string $cast): int {}

    protected function getIntCastKind(string $cast): int {}

    protected function getBoolCastKind(string $cast): int {}

    protected function getStringCastKind(string $cast): int {}

    /** @param array<string, mixed> $attributes */
    protected function parseLNumber(string $str, array $attributes, bool $allowInvalidOctal = false): Int_ {}

    /**
     * Parse a T_NUM_STRING token into either an integer or string node.
     *
     * @param string $str Number string
     * @param array<string, mixed> $attributes Attributes
     *
     * @return Int_|String_ Integer or string node.
     */
    protected function parseNumString(string $str, array $attributes) {}

    /** @param array<string, mixed> $attributes */
    protected function stripIndentation(
        string $string, int $indentLen, string $indentChar,
        bool $newlineAtStart, bool $newlineAtEnd, array $attributes
    ): string {}

    /**
     * @param string|(Expr|InterpolatedStringPart)[] $contents
     * @param array<string, mixed> $attributes
     * @param array<string, mixed> $endTokenAttributes
     */
    protected function parseDocString(
        string $startToken, $contents, string $endToken,
        array $attributes, array $endTokenAttributes, bool $parseUnicodeEscape
    ): Expr {}

    protected function createCommentFromToken(Token $token, int $tokenPos): Comment {}

    /**
     * Get last comment before the given token position, if any
     */
    protected function getCommentBeforeToken(int $tokenPos): ?Comment {}

    /**
     * Create a zero-length nop to capture preceding comments, if any.
     */
    protected function maybeCreateZeroLengthNop(int $tokenPos): ?Nop {}

    protected function maybeCreateNop(int $tokenStartPos, int $tokenEndPos): ?Nop {}

    protected function handleHaltCompiler(): string {}

    protected function inlineHtmlHasLeadingNewline(int $stackPos): bool {}

    /**
     * @return array<string, mixed>
     */
    protected function createEmptyElemAttributes(int $tokenPos): array {}

    protected function fixupArrayDestructuring(Array_ $node): Expr\List_ {}

    protected function postprocessList(Expr\List_ $node): void {}

    /** @param ElseIf_|Else_ $node */
    protected function fixupAlternativeElse($node): void {}

    protected function checkClassModifier(int $a, int $b, int $modifierPos): void {}

    protected function checkModifier(int $a, int $b, int $modifierPos): void {}

    protected function checkParam(Param $node): void {}

    protected function checkTryCatch(TryCatch $node): void {}

    protected function checkNamespace(Namespace_ $node): void {}

    private function checkClassName(?Identifier $name, int $namePos): void {}

    /** @param Name[] $interfaces */
    private function checkImplementedInterfaces(array $interfaces): void {}

    protected function checkClass(Class_ $node, int $namePos): void {}

    protected function checkInterface(Interface_ $node, int $namePos): void {}

    protected function checkEnum(Enum_ $node, int $namePos): void {}

    protected function checkClassMethod(ClassMethod $node, int $modifierPos): void {}

    protected function checkClassConst(ClassConst $node, int $modifierPos): void {}

    protected function checkUseUse(UseItem $node, int $namePos): void {}

    protected function checkPropertyHooksForMultiProperty(Property $property, int $hookPos): void {}

    /** @param PropertyHook[] $hooks */
    protected function checkEmptyPropertyHookList(array $hooks, int $hookPos): void {}

    protected function checkPropertyHook(PropertyHook $hook, ?int $paramListPos): void {}

    protected function checkPropertyHookModifiers(int $a, int $b, int $modifierPos): void {}

    protected function checkConstantAttributes(Const_ $node): void {}

    protected function checkPipeOperatorParentheses(Expr $node): void {}

    /**
     * @param Property|Param $node
     */
    protected function addPropertyNameToHooks(Node $node): void {}

    /** @param array<Node\Arg|Node\VariadicPlaceholder|Node\ArgPlaceholder> $args */
    private function isSimpleExit(array $args): bool {}

    /**
     * @param array<Node\Arg|Node\VariadicPlaceholder|Node\ArgPlaceholder> $args
     * @param array<string, mixed> $attrs
     */
    protected function createExitExpr(string $name, int $namePos, array $args, array $attrs): Expr {}

    /**
     * Creates the token map.
     *
     * The token map maps the PHP internal token identifiers
     * to the identifiers used by the Parser. Additionally it
     * maps T_OPEN_TAG_WITH_ECHO to T_ECHO and T_CLOSE_TAG to ';'.
     *
     * @return array<int, int> The token map
     */
    protected function createTokenMap(): array {}
}
