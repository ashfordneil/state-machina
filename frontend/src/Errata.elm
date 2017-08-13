module Errata exposing (..)

import Html exposing (..)
import Html.Attributes exposing (href, target)


stateMachines =
    div []
        [ h2 [] [ text "Information" ]
        , h3 [] [ text "Authors" ]
        , p [] [ text "Proudly made by Neil Ashford, Simon Gordon, Callum Hays and Damian Van Kranendonk." ]
        , h3 [] [ text "About this application" ]
        , div []
            [ p [] [ text "State Machina is an automated tool to assist in the design of finite state machines." ]
            , p []
                [ text
                    ("The green state is the starting state of your machine. It cannot be deleted. "
                        ++ "Red states are the valid accepting / final states of your machine. "
                        ++ "The blue states are the other (non-accepting) states. "
                    )
                ]
            , p []
                [ text
                    ("Transitions between states are represented by arrows. The label \"input 1/input 2\" on "
                        ++ "a transition means that that transition can take place on input 1 or input 2."
                    )
                ]
            ]
        , h3 [] [ text "Resources" ]
        , ul []
            [ li []
                [ a
                    [ href "http://blog.markshead.com/869/state-machines-computer-science/"
                    , target "_blank"
                    ]
                    [ text "State Machines - Basics of Comuter Science"
                    ]
                ]
            , li []
                [ a
                    [ href "https://github.com/ashfordneil/state-machina/"
                    , target "_blank"
                    ]
                    [ text "Source code of this application"
                    ]
                ]
                [ text "State Machines - Basics of Computer Science (Blog)" ]
            ]
        ]
