-- Do not manually edit this file, it was auto-generated by Graphqelm
-- https://github.com/dillonkearns/graphqelm


module ApiPub.Object.MutationError exposing (..)

import ApiPub.InputObject
import ApiPub.Interface
import ApiPub.Object
import ApiPub.Scalar
import ApiPub.Union
import Graphqelm.Field as Field exposing (Field)
import Graphqelm.Internal.Builder.Argument as Argument exposing (Argument)
import Graphqelm.Internal.Builder.Object as Object
import Graphqelm.Internal.Encode as Encode exposing (Value)
import Graphqelm.OptionalArgument exposing (OptionalArgument(Absent))
import Graphqelm.SelectionSet exposing (SelectionSet)
import Json.Decode as Decode


{-| Select fields to build up a SelectionSet for this object.
-}
selection : (a -> constructor) -> SelectionSet (a -> constructor) ApiPub.Object.MutationError
selection constructor =
    Object.selection constructor


key : Field String ApiPub.Object.MutationError
key =
    Object.fieldDecoder "key" [] Decode.string


messages : Field (List String) ApiPub.Object.MutationError
messages =
    Object.fieldDecoder "messages" [] (Decode.string |> Decode.list)