<?php
namespace AliasProbe;

use AliasProbe\Marker as ImportedMarker;
use function class_alias as make_alias;

$GLOBALS['aliasResults']['direct'] = \class_alias(Marker::class, Alias::class);
$GLOBALS['aliasResults']['imported'] = \class_alias(ImportedMarker::class, ImportedAlias::class);
$GLOBALS['aliasResults']['string'] = \class_alias('AliasProbe\Marker', 'AliasProbe\StringAlias');
$GLOBALS['aliasResults']['bare-string'] = @\class_alias('Marker', 'AliasProbe\BareString');
$GLOBALS['aliasResults']['chain'] = \class_alias(Alias::class, Chain::class);
$GLOBALS['aliasResults']['meta'] = \class_alias(\Attribute::class, Meta::class);
$GLOBALS['aliasResults']['function-import'] = make_alias(Marker::class, FunctionImport::class);
$GLOBALS['aliasResults']['clash'] = @\class_alias(Marker::class, Clash::class);
