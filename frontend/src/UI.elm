module UI exposing (..)

import Html exposing (..)
import Html.Attributes exposing (class, disabled, id)
import Html.Events exposing (onClick)
import Ports.Vis.Network as Network exposing (NodeId, Edge)


type Msg
    = EdgeSelected Edge
    | Go
    | UpdateEdge Edge
    | AddState
    | AddTransition
    | Selected String
    | Deselect


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

        ( Deselect, EditTransition tr ) ->
            ( EditTransition { tr | selected = Nothing }, Cmd.none )

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
        selected =
            case tr.selected of
                Just _ ->
                    True

                _ ->
                    False

        letters =
            div []
                ((tr.symbols
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
                 )
                    ++ [ button
                            [ onClick Deselect
                            , disabled (not selected)
                            , class "button"
                            ]
                            [ text "Cancel" ]
                       ]
                )
    in
        div [ id "edit-transition" ]
            [ h2 [] [ text "Edit Transition" ]
            , letters
            , div [ class "footer" ]
                [ button
                    [ class "button"
                    , disabled selected
                    ]
                    [ text "+ add symbol" ]
                , button
                    [ class "button"
                    , disabled (not selected)
                    , onClick
                        (UpdateEdge
                            { from = tr.from
                            , to = tr.to
                            , label = String.join "/" (List.filter (\x -> Just x /= tr.selected) tr.symbols)
                            }
                        )
                    ]
                    [ text "- remove symbol" ]
                ]
            ]
