module UI exposing (..)

import Html exposing (Html, div, button, text)
import Html.Attributes exposing (class, id)
import Html.Events exposing (onClick)
import Ports.Vis.Network as Network exposing (NodeId, Edge)


type Msg
    = EdgeSelected Edge
    | Go
    | AddState
    | AddTransition
    | Selected String


type Model
    = Unselected
    | EditTransition Transition


type alias Transition =
    { from : NodeId
    , to : NodeId
    , symbols : List String
    , selected : Maybe String
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( EdgeSelected edge, _ ) ->
            ( EditTransition
                { from = edge.from
                , to = edge.to
                , symbols = String.split "/" edge.label
                , selected = Nothing
                }
            , Cmd.none
            )

        ( Selected sym, EditTransition tr ) ->
            ( EditTransition { tr | selected = Just sym }, Cmd.none )

        _ ->
            ( model, Cmd.none )


viewButtons : Model -> Html Msg
viewButtons model =
    div [ class "controls" ]
        [ button [ onClick AddState ] [ text "Add State" ]
        , button [ onClick AddTransition ] [ text "Add Transition" ]
        , button [ onClick Go ] [ text "Go" ]
        ]


viewInput : Model -> Html Msg
viewInput model =
    case model of
        Unselected ->
            div [] []

        EditTransition tr ->
            editTransition tr


editTransition : Transition -> Html Msg
editTransition tr =
    let
        letters =
            tr.symbols
                |> List.map
                    (\sym ->
                        let
                            selected =
                                (case tr.selected of
                                    Just sym2 ->
                                        sym == sym2

                                    _ ->
                                        False
                                )

                            style =
                                if selected then
                                    [ class "selected-button" ]
                                else
                                    [ class "button" ]
                        in
                            button (style ++ [ onClick (Selected sym) ]) [ text sym ]
                    )
    in
        div [id "edit-transition"] letters
