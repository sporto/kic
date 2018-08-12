-- Do not manually edit this file, it was auto-generated by Graphqelm
-- https://github.com/dillonkearns/graphqelm


module ApiPub.Object.SignUpResponse exposing (..)

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
selection : (a -> constructor) -> SelectionSet (a -> constructor) ApiPub.Object.SignUpResponse
selection constructor =
    Object.selection constructor


success : Field Bool ApiPub.Object.SignUpResponse
success =
    Object.fieldDecoder "success" [] Decode.bool


errors : SelectionSet decodesTo ApiPub.Object.MutationError -> Field (List decodesTo) ApiPub.Object.SignUpResponse
errors object =
    Object.selectionField "errors" [] object (identity >> Decode.list)


token : Field (Maybe String) ApiPub.Object.SignUpResponse
token =
    Object.fieldDecoder "token" [] (Decode.string |> Decode.nullable)