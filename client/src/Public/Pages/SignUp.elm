module Public.Pages.SignUp exposing
    ( Model
    , Msg
    , init
    , subscriptions
    , update
    , view
    )

import ApiPub.InputObject exposing (SignUp)
import ApiPub.Mutation
import ApiPub.Object
import ApiPub.Object.SignUpResponse
import Browser
import Graphql.Operation exposing (RootMutation, RootQuery)
import Graphql.SelectionSet as SelectionSet exposing (SelectionSet, with)
import Html exposing (..)
import Html.Attributes exposing (class, href, name, type_, value)
import Html.Events exposing (onClick, onInput, onSubmit)
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import Public.Pages.Common as Common
import RemoteData
import Shared.Actions as Actions
import Shared.Css exposing (molecules)
import Shared.Globals exposing (..)
import Shared.GraphQl exposing (GraphData, GraphResponse, MutationError, mutationErrorPublicSelection, sendPublicMutation)
import Shared.Routes as Routes
import Shared.Sessions as Sessions
import String.Verify
import UI.Flash as Flash
import UI.Forms as Forms
import UI.Icons as Icons
import UI.PublicLinks as PublicLinks
import Verify exposing (Validator, validate, verify)


type alias Model =
    { form : SignUp
    , response : GraphData SignUpResponse
    , validationErrors : Maybe ( ValidationError, List ValidationError )
    }


type alias ValidationError =
    ( Field, String )


newModel : Flags -> Model
newModel flags =
    { form = Sessions.newSignUp
    , response = RemoteData.NotAsked
    , validationErrors = Nothing
    }


type alias SignUpResponse =
    { success : Bool
    , errors : List MutationError
    , jwt : Maybe String
    }


asFormInModel : Model -> SignUp -> Model
asFormInModel model form =
    { model | form = form }


type alias Returns =
    ( Model, Cmd Msg, Actions.Actions Msg )


init : PublicContext -> Returns
init context =
    ( newModel context.flags
    , Cmd.none
    , Actions.none
    )


type Msg
    = ChangeEmail String
    | ChangeName String
    | ChangeUsername String
    | ChangePassword String
    | Submit
    | OnSubmitResponse (GraphResponse SignUpResponse)


update : PublicContext -> Msg -> Model -> Returns
update context msg model =
    case msg of
        ChangeEmail email ->
            ( email
                |> Sessions.asEmailInSignUp model.form
                |> asFormInModel model
            , Cmd.none
            , Actions.none
            )

        ChangeName name ->
            ( name
                |> Sessions.asNameInSignUp model.form
                |> asFormInModel model
            , Cmd.none
            , Actions.none
            )

        ChangeUsername name ->
            ( name
                |> Sessions.asUsernameInSignUp model.form
                |> asFormInModel model
            , Cmd.none
            , Actions.none
            )

        ChangePassword password ->
            ( password
                |> Sessions.asPasswordInSignUp model.form
                |> asFormInModel model
            , Cmd.none
            , Actions.none
            )

        Submit ->
            case validateForm model.form of
                Err errors ->
                    ( { model
                        | validationErrors = Just errors
                      }
                    , Cmd.none
                    , Actions.none
                    )

                Ok input ->
                    ( { model
                        | response = RemoteData.Loading
                        , validationErrors = Nothing
                      }
                    , sendCreateSignUpMutation context input
                    , Actions.none
                    )

        OnSubmitResponse result ->
            case result of
                Err e ->
                    ( { model | response = RemoteData.Failure e }
                    , Cmd.none
                    , Actions.addErrorNotification
                        "Something went wrong"
                    )

                Ok response ->
                    case response.jwt of
                        Just jwt ->
                            ( { model | response = RemoteData.Success response }
                            , Cmd.none
                            , Actions.startSession jwt
                            )

                        Nothing ->
                            ( { model | response = RemoteData.Success response }
                            , Cmd.none
                            , Actions.none
                            )


subscriptions model =
    Sub.none


type Field
    = Field_Name
    | Field_Email
    | Field_Username
    | Field_Password



-- VIEW


view : PublicContext -> Model -> Html Msg
view context model =
    Common.layout
        context
        { containerAttributes = [ class "w-80" ]
        }
        [ Forms.form_ (formArgs model)
        ]


formArgs : Model -> Forms.Args SignUpResponse Msg
formArgs model =
    { title = "Sign up"
    , intro = Nothing
    , submitContent = [ text "Sign up" ]
    , fields = formFields model
    , onSubmit = Submit
    , response = model.response
    }


formFields model =
    [ Forms.set
        Field_Name
        "Name"
        (input
            [ class molecules.form.input
            , onInput ChangeName
            , name "name"
            , value model.form.name
            ]
            []
        )
        model.validationErrors
    , Forms.set
        Field_Username
        "Username"
        (input
            [ class molecules.form.input
            , onInput ChangeUsername
            , name "username"
            , value model.form.username
            ]
            []
        )
        model.validationErrors
    , Forms.set
        Field_Email
        "Email"
        (input
            [ class molecules.form.input
            , onInput ChangeEmail
            , name "email"
            , value model.form.email
            ]
            []
        )
        model.validationErrors
    , Forms.set
        Field_Password
        "Password"
        (input
            [ class molecules.form.input
            , type_ "password"
            , onInput ChangePassword
            , name "password"
            , value model.form.password
            ]
            []
        )
        model.validationErrors
    ]


maybeErrors : Model -> Html msg
maybeErrors model =
    case model.response of
        RemoteData.Success response ->
            if List.isEmpty response.errors then
                text ""

            else
                Forms.mutationError
                    "other"
                    response.errors

        RemoteData.Failure err ->
            Flash.error "Error"

        _ ->
            text ""



-- Validations


validateForm : Validator ValidationError SignUp SignUp
validateForm =
    validate SignUp
        |> verify .name (Forms.verifyName Field_Name)
        |> verify .username (Forms.verifyUsername Field_Username)
        |> verify .email (Forms.verifyEmail Field_Email)
        |> verify .password (Forms.verifyPassword Field_Password)



-- GraphQL data


sendCreateSignUpMutation : PublicContext -> SignUp -> Cmd Msg
sendCreateSignUpMutation context form =
    sendPublicMutation
        context
        "create-sign-up"
        (createSignUpMutation form)
        OnSubmitResponse


createSignUpMutation : SignUp -> SelectionSet SignUpResponse RootMutation
createSignUpMutation form =
    SelectionSet.succeed identity
        |> with
            (ApiPub.Mutation.signUp
                { signUp = form }
                formResponseSelection
            )


formResponseSelection : SelectionSet SignUpResponse ApiPub.Object.SignUpResponse
formResponseSelection =
    SelectionSet.succeed SignUpResponse
        |> with ApiPub.Object.SignUpResponse.success
        |> with (ApiPub.Object.SignUpResponse.errors mutationErrorPublicSelection)
        |> with ApiPub.Object.SignUpResponse.jwt
