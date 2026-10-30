<?php
namespace AliasProbe;

#[Alias]
function direct(): void {}
#[aLiAs]
function variant(): void {}
#[ImportedAlias]
function imported(): void {}
#[StringAlias]
function stringAlias(): void {}
#[BareString]
function bareString(): void {}
#[Chain]
function chain(): void {}
#[FunctionImport]
function functionImport(): void {}

new Alias();
Alias::ping();
Alias::VALUE;
function signature(Alias $value): Alias { return $value; }

#[Alias, Alias]
function repeat(): void {}
#[Alias, aLiAs]
function repeatCase(): void {}
#[Marker, Alias]
function separateNames(): void {}
#[Alias]
class WrongTarget {}

#[Clash]
function clash(): void {}

#[ViaImport]
class ViaImportUse {}
#[ViaLowerNative]
class ViaLowerNativeUse {}
#[ViaMeta]
class ViaMetaUse {}

#[\AliasProbe\marker]
function directCaseControl(): void {}
