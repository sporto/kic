module Admin.Pages.InviteAdmin exposing (InvitationResponse, Model, Msg(..), createMutation, createMutationCmd, init, invitationResponseSelection, newModel, submit, subscriptions, update, view)

import Api.Mutation
import Api.Object
import Api.Object.InvitationResponse
import Graphql.Operation exposing (RootMutation, RootQuery)
import Graphql.SelectionSet exposing (SelectionSet, with)
import Html exposing (..)
import Html.Attributes exposing (class, href, name, style, type_, value)
import Html.Events exposing (onInput, onSubmit)
import RemoteData
import Shared.Actions as Actions
import Shared.Css exposing (molecules)
import Shared.Globals exposing (..)
import Shared.GraphQl exposing (GraphData, GraphResponse, MutationError, mutationErrorSelection, sendMutation)
import UI.Flash as Flash
import UI.Icons as Icons


type Msg
    = ChangeEmail String
    | Submit
    | OnSubmitResponse (GraphResponse InvitationResponse)


type alias Model =
    { email : String
    , response : GraphData InvitationResponse
    }


newModel : Model
newModel =
    { email = ""
    , response = RemoteData.NotAsked
    }


type alias InvitationResponse =
    { success : Bool
    , errors : List MutationError
    }


type alias Returns =
    ( Model, Cmd Msg, Actions.Actions Msg )


init : Returns
init =
    ( newModel, Cmd.none, Actions.none )


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


update : Context -> Msg -> Model -> Returns
update context msg model =
    case msg of
        ChangeEmail email ->
            ( { model | email = email }
            , Cmd.none
            , Actions.none
            )

        Submit ->
            ( { model | response = RemoteData.Loading }
            , createMutationCmd context model.email
            , Actions.none
            )

        OnSubmitResponse result ->
            case result of
                Err e ->
                    ( { model | response = RemoteData.Failure e }
                    , Cmd.none
                    , Actions.none
                    )

                Ok response ->
                    if response.success then
                        ( { model
                            | response = RemoteData.Success response
                            , email = ""
                          }
                        , Cmd.none
                        , Actions.none
                        )

                    else
                        ( { model | response = RemoteData.Success response }
                        , Cmd.none
                        , Actions.none
                        )


view : Context -> Model -> Html Msg
view context model =
    section [ class molecules.page.container, class "flex justify-center" ]
        [ div [ style "width" "24rem" ]
            [ h1 [ class molecules.page.title ] [ text "Invite admin" ]
            , form [ class "mt-2", onSubmit Submit ]
                [ p [ class "text-grey-dark leading-normal" ]
                    [ text "You can invite another parent to manage these accounts with you."
                    ]
                , flash model
                , p [ class molecules.form.fieldset ]
                    [ label
                        [ class molecules.form.label
                        ]
                        [ text "Email" ]
                    , input
                        [ class molecules.form.input
                        , type_ "email"
                        , name "email"
                        , value model.email
                        , onInput ChangeEmail
                        ]
                        []
                    ]
                , p [ class molecules.form.actions ]
                    [ submit model
                    ]
                ]
            ]
        ]


submit : Model -> Html Msg
submit model =
    case model.response of
        RemoteData.Loading ->
            Icons.spinner

        _ ->
            button [ class molecules.form.submit ] [ i [ class "fas fa-envelope mr-2" ] [], text "Invite" ]


flash : Model -> Html msg
flash model =
    case model.response of
        RemoteData.Success response ->
            if response.success then
                Flash.success
                    "The invitation was sent"

            else
                text ""

        RemoteData.Failure e ->
            Flash.error
                "Something went wrong"

        _ ->
            text ""



-- GraphQl


createMutationCmd : Context -> String -> Cmd Msg
createMutationCmd context email =
    sendMutation
        context
        "create-invitation"
        (createMutation email)
        OnSubmitResponse


createMutation : String -> SelectionSet InvitationResponse RootMutation
createMutation email =
    Api.Mutation.selection identity
        |> with
            (Api.Mutation.inviteAdmin
                { input = { email = email } }
                invitationResponseSelection
            )


invitationResponseSelection : SelectionSet InvitationResponse Api.Object.InvitationResponse
invitationResponseSelection =
    Api.Object.InvitationResponse.selection InvitationResponse
        |> with Api.Object.InvitationResponse.success
        |> with (Api.Object.InvitationResponse.errors mutationErrorSelection)