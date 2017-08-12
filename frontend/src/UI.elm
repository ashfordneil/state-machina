module UI exposing (..)

import Html exposing (Html, div, button, text)
import Html.Attributes exposing (class)
import Html.Events exposing (onClick)


type Msg
    = AddState
    | AddStateResult
    | AddTransition
    | AddTransitionResult
    | Go


type Model
    = Unselected
    | AddingState StateForm
    | AddingTransition


type alias StateForm =
    { name : String
    , accepting : Bool
    }


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
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
    div [] []
