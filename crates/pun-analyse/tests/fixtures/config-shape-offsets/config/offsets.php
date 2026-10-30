<?php

/**
 * @return array{
 *   string,
 *   array<int>,
 *   5: string,
 *   array<int>,
 *   string,
 *   named: string,
 *   nested: array{array<int>, string, 4: array<int>, string},
 *   optional?: array{string, array<int>},
 *   constants: array{\PDO::PARAM_NULL: string, \PDO::PARAM_INT: array<int>, string},
 *   namedConstant: array{\DateTimeInterface::ATOM: string, string},
 *   optionalInteger: array{4?: string, string, array<int>, tail: string},
 *   negativeInteger: array{-3: string, array<int>, string},
 *   quotedInteger: array{'5': array<int>, string},
 *   duplicate: array{0: array<int>, 0: string, array<int>},
 *   missingConstant: array{\ConfigShapeOffsets\Missing::KEY: string, known: string}
 * }
 */
return [];
