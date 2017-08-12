module Main exposing (..)

import Html exposing (..)
import Ports.Vis.Network as Network exposing (Network)


type Msg
    = Network Network.Msg


type alias Model =
    { network : Network
    }


init : ( Model, Cmd Msg )
init =
    let
        startColor =
            "#4BAE4F"

        normalColor =
            "#03A9F4"

        finishColor =
            "#F34236"

        network =
            { divId = "mainNetwork"
            , data =
                { nodes =
                    [ { id = 0, label = "Start", color = startColor }
                    , { id = 1, label = "1", color = normalColor }
                    , { id = 2, label = "2", color = normalColor }
                    , { id = 3, label = "3", color = finishColor }
                    ]
                , edges =
                    [ { from = 0, to = 1, label = "" }
                    , { from = 1, to = 2, label = "a" }
                    , { from = 2, to = 3, label = "a/b" }
                    , { from = 1, to = 1, label = "a/b" }
                    ]
                }
            , options =
                let
                    defaultOptions =
                        Network.defaultOptions

                    defaultEdgesOptions =
                        defaultOptions.edges

                    defaultLayoutOptions =
                        defaultOptions.layout
                in
                { defaultOptions
                    | height = "1000px"
                    , edges = { defaultEdgesOptions | arrows = Just "to" }
                    , layout = { defaultLayoutOptions | randomSeed = Just 0 }
                }
            }
    in
    { network = network
    }
        ! [ Cmd.map Network (Network.initCmd network) ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Network msg ->
            let
                ( networkModel, networkCmd ) =
                    Network.update msg model.network
            in
            { model | network = networkModel }
                ! [ Cmd.map Network networkCmd ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.map Network (Network.subscriptions model.network)



---- VIEW ----


view : Model -> Html Msg
view model =
    Network.view model.network



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = subscriptions
        }
