<?php
// API oracle for crates/pun-tokens/tests/api.rs: runs each case's ops on PHP-CS-Fixer's Tokens / Token /
// TokensAnalyzer and prints one JSON line per case (a list with one result per op; exceptions are
// {"throws": true}). Input (stdin): the JSON case list from tests/api_cases.json. Op reference: tests/api.rs.
use PhpCsFixer\Tokenizer\Token;
use PhpCsFixer\Tokenizer\Tokens;
use PhpCsFixer\Tokenizer\TokensAnalyzer;

error_reporting(E_ALL & ~E_DEPRECATED);
set_error_handler(static function (int $no, string $msg): bool { throw new ErrorException($msg, 0, $no); });
require (getenv('PUN_PHP_CS_FIXER') ?: __DIR__.'/../../../.oracle/php-cs-fixer').'/vendor/autoload.php';

function kid(string $n): int { return str_starts_with($n, 'CT::') ? constant('PhpCsFixer\\Tokenizer\\'.$n) : constant($n); }
function kname(?int $id): ?string { return null === $id ? null : Token::getNameForId($id); }
/** "x" -> "x", ["T_X"] -> [T_X], ["T_X", "c"] -> [T_X, "c"] */
function proto($p) { return is_string($p) ? $p : (count($p) === 1 ? [kid($p[0])] : [kid($p[0]), $p[1]]); }
function protos(array $ps): array { return array_map('proto', $ps); }
function tok($p): Token { return new Token(proto($p)); }
function toks(array $ps): array { return array_map('tok', $ps); }
function key_($k) { return is_string($k) ? $k : kid($k[0]); }
function kinds($k) { return is_string($k) ? kid($k) : array_map('kid', $k); }
function cs($c) { return is_array($c) ? array_map(fn ($v) => (bool) $v, $c) : $c; }
function named(array $byKind): array { $o = []; foreach ($byKind as $k => $v) { $o[kname($k)] = array_keys($v); } return $o; }

function op(Tokens &$t, array $o) {
    $a = new TokensAnalyzer($t);
    $name = array_shift($o);
    $x = $o;
    switch ($name) {
        case 'code': return $t->generateCode();
        case 'count': $n = $t->count(); return [$n, $n, 0 === $n, $n, iterator_count($t)];
        case 'contents': return array_map(fn (Token $k) => $k->getContent(), $t->toArray());
        case 'changed': return $t->isChanged();
        case 'clearChanged': $t->clearChanged(); return null;
        case 'partial': return $t->generatePartialCode($x[0], $x[1]);
        case 'detect': $r = Tokens::detectBlockType($t[$x[0]]); return null === $r ? null : [$r['type'], $r['isStart']];
        case 'blocks': // the opposite edge of a block edge token, else null
            $r = Tokens::detectBlockType($t[$x[0]]);
            return null === $r ? null : ($r['isStart'] ? $t->findBlockEnd($r['type'], $x[0]) : $t->findBlockStart($r['type'], $x[0]));
        case 'blockEnd': return $t->findBlockEnd($x[0], $x[1]);
        case 'blockStart': return $t->findBlockStart($x[0], $x[1]);
        case 'givenKind':
            $r = $t->findGivenKind(kinds($x[0]), $x[1] ?? 0, $x[2] ?? null);
            return is_string($x[0]) ? array_keys($r) : named($r);
        case 'nextNonWs': return $t->getNextNonWhitespace($x[0], $x[1] ?? null);
        case 'prevNonWs': return $t->getPrevNonWhitespace($x[0], $x[1] ?? null);
        case 'nonWsSibling': return $t->getNonWhitespaceSibling($x[0], $x[1], $x[2] ?? null);
        case 'nextOfKind': return $t->getNextTokenOfKind($x[0], protos($x[1]), $x[2] ?? true);
        case 'prevOfKind': return $t->getPrevTokenOfKind($x[0], protos($x[1]), $x[2] ?? true);
        case 'ofKindSibling': return $t->getTokenOfKindSibling($x[0], $x[1], protos($x[2]), $x[3] ?? true);
        case 'notOfKindSibling': return $t->getTokenNotOfKindSibling($x[0], $x[1], protos($x[2]));
        case 'notOfKindsSibling': return $t->getTokenNotOfKindsSibling($x[0], $x[1], array_map('kid', $x[2]));
        case 'meaningfulSibling': return $t->getMeaningfulTokenSibling($x[0], $x[1]);
        case 'nonEmptySibling': return $t->getNonEmptySibling($x[0], $x[1]);
        case 'nextMeaningful': return $t->getNextMeaningfulToken($x[0]);
        case 'prevMeaningful': return $t->getPrevMeaningfulToken($x[0]);
        case 'sequence':
            $r = $t->findSequence(protos($x[0]), $x[1] ?? 0, $x[2] ?? null, cs($x[3] ?? true));
            return null === $r ? null : array_keys($r);
        case 'insertAt': $t->insertAt($x[0], toks($x[1])); return null;
        case 'insertSlices': $s = []; foreach ($x[0] as $i => $l) { $s[(int) $i] = toks($l); } $t->insertSlices($s); return null;
        case 'emptyAt': return $t->isEmptyAt($x[0]);
        case 'clearAt': $t->clearAt($x[0]); return null;
        case 'clearMerge': $t->clearTokenAndMergeSurroundingWhitespace($x[0]); return null;
        case 'set': $t[$x[0]] = tok($x[1]); return null;
        case 'overrideRange': $t->overrideRange($x[0], $x[1], toks($x[2])); return null;
        case 'removeLeadingWs': $t->removeLeadingWhitespace($x[0], $x[1] ?? null); return null;
        case 'removeTrailingWs': $t->removeTrailingWhitespace($x[0], $x[1] ?? null); return null;
        case 'setCode': $t->setCode($x[0]); return null;
        case 'transform': PhpCsFixer\Tokenizer\Transformers::createSingleton()->transform($t); return null;
        case 'fromArray': $t = Tokens::fromArray(toks($x[0])); return null;
        case 'allFound': return $t->isAllTokenKindsFound(array_map('kid', $x[0]));
        case 'anyFound': return $t->isAnyTokenKindsFound(array_map('kid', $x[0]));
        case 'found': return $t->isTokenKindFound(key_($x[0]));
        case 'countKind': return $t->countTokenKind(key_($x[0]));
        case 'clearRange': $t->clearRange($x[0], $x[1]); return null;
        case 'monolithic': return $t->isMonolithicPhp();
        case 'multiline': return $t->isPartialCodeMultiline($x[0], $x[1]);
        case 'altSyntax': return $t->hasAlternativeSyntax();
        case 'clearMerge': $t->clearTokenAndMergeSurroundingWhitespace($x[0]); return null;
        case 'ensureWs': return $t->ensureWhitespaceAtIndex($x[0], $x[1], $x[2]);
        case 'clearEmpty': $t->clearEmptyTokens(); return null;
        // Token
        case 'tok':
            $k = $t[$x[0]];
            return [$k->getName(), kname($k->getId()), $k->getContent(), $k->isArray(), $k->isCast(), $k->isClassy(), $k->isComment(),
                $k->isObjectOperator(), $k->isKeyword(), $k->isMagicConstant(), $k->isWhitespace()];
        case 'isWs': return $t[$x[0]]->isWhitespace($x[1] ?? null);
        case 'equals': return $t[$x[0]]->equals(proto($x[1]), $x[2] ?? true);
        case 'equalsProto': return $t[$x[0]]->equals($t[$x[1]]->getPrototype(), $x[2] ?? true);
        case 'equalsToken': return $t[$x[0]]->equals($t[$x[1]], $x[2] ?? true);
        case 'newEquals': return tok($x[0])->equals(proto($x[1]), $x[2] ?? true);
        case 'newEqualsToken': return tok($x[0])->equals(tok($x[1]), $x[2] ?? true);
        case 'equalsAny': return $t[$x[0]]->equalsAny(protos($x[1]), $x[2] ?? true);
        case 'isGivenKind': return $t[$x[0]]->isGivenKind(kinds($x[1]));
        // TokensAnalyzer
        case 'classyElements': $o = []; foreach ($a->getClassyElements() as $i => $e) { $o[] = [$i, $e['type'], $e['classIndex']]; } return $o;
        case 'classyModifiers': $m = $a->getClassyModifiers($x[0]); return [$m['final'], $m['abstract'], $m['readonly']];
        case 'importUses': return $x[0] ? (object) $a->getImportUseIndexes(true) : $a->getImportUseIndexes(false);
        case 'isArray': return $a->isArray($x[0]);
        case 'arrayMultiLine': return $a->isArrayMultiLine($x[0]);
        case 'blockMultiline': return $a->isBlockMultiline($t, $x[0]);
        case 'methodAttributes': $m = $a->getMethodAttributes($x[0]); return [kname($m['visibility']), $m['static'], $m['abstract'], $m['final']];
        case 'anonClass': return $a->isAnonymousClass($x[0]);
        case 'lambda': return $a->isLambda($x[0]);
        case 'arrowEnd': return $a->getLastTokenIndexOfArrowFunction($x[0]);
        case 'unarySucc': return $a->isUnarySuccessorOperator($x[0]);
        case 'unaryPred': return $a->isUnaryPredecessorOperator($x[0]);
        case 'binary': return $a->isBinaryOperator($x[0]);
        case 'doWhile': return $a->isWhilePartOfDoWhile($x[0]);
        case 'enumCase': return $a->isEnumCase($x[0]);
        case 'superGlobal': return $a->isSuperGlobal($x[0]);
    }
    throw new LogicException("unknown op $name");
}

foreach (json_decode(stream_get_contents(STDIN), true, 512, JSON_THROW_ON_ERROR) as $case) {
    Tokens::clearCache();
    $t = Tokens::fromCode($case['code']);
    $out = [];
    foreach ([...$case['ops'], ...(isset($case['noTail']) ? [] : [['code'], ['changed']])] as $o) {
        if ($o[0] === 'each') {
            $r = [];
            for ($i = 0, $n = $t->count(); $i < $n; ++$i) {
                try { $r[] = op($t, [$o[1], $i, ...array_slice($o, 2)]); } catch (Throwable $e) { $r[] = ['throws' => true]; }
            }
            $out[] = $r;
            continue;
        }
        try { $out[] = op($t, $o); } catch (Throwable $e) { $out[] = ['throws' => true]; }
    }
    echo json_encode($out, JSON_INVALID_UTF8_SUBSTITUTE), "\n";
}
