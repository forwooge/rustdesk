lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Estat"),
        ("Your Desktop", "El teu escriptori"),
        ("desk_tip", "Pots accedir al teu escriptori amb aquest ID i contrasenya."),
        ("Password", "Contrasenya"),
        ("Ready", "Llest"),
        ("Established", "Establert"),
        ("connecting_status", "Connexió a la xarxa RustDesk en progrés..."),
        ("Enable service", "Habilita el servei"),
        ("Start service", "Inicia el servei"),
        ("Service is running", "El servei s'està executant"),
        ("Service is not running", "El servei no s'està executant"),
        ("not_ready_status", "No està llest. Comprova la teva connexió"),
        ("Control Remote Desktop", "Controlar escriptori remot"),
        ("Transfer file", "Transferir arxiu"),
        ("Connect", "Connectar"),
        ("Recent sessions", "Sessions recents"),
        ("Address book", "Directori"),
        ("Confirmation", "Confirmació"),
        ("TCP tunneling", "Túnel TCP"),
        ("Remove", "Eliminar"),
        ("Refresh random password", "Actualitza la contrasenya aleatòria"),
        ("Set your own password", "Estableix la teva contrasenya"),
        ("Enable keyboard/mouse", "Habilita el teclat/ratolí"),
        ("Enable clipboard", "Habilita el portapapers"),
        ("Enable file transfer", "Habilita la transferència d'arxius"),
        ("Enable TCP tunneling", "Habilita el túnel TCP"),
        ("IP Whitelisting", "Adreces IP admeses"),
        ("ID/Relay Server", "Servidor ID/Relay"),
        ("Import server config", "Importa la configuració del servidor"),
        ("Export Server Config", "Exporta la configuració del servidor"),
        ("Import server configuration successfully", "Configuració del servidor importada amb èxit"),
        ("Export server configuration successfully", "Configuració del servidor exportada con èxit"),
        ("Invalid server configuration", "Configuració del servidor incorrecta"),
        ("Clipboard is empty", "El portapapers està buit"),
        ("Stop service", "Atura el servei"),
        ("Change ID", "Canviar ID"),
        ("Your new ID", "La teva nova ID"),
        ("length %min% to %max%", "Ha de tenir entre %min% i %max% caràcters"),
        ("starts with a letter", "comença amb una lletra"),
        ("allowed characters", "caràcters permesos"),
        ("id_change_tip", "Només pots utilitzar caràcters a-z, A-Z, 0-9 e _ (guionet baix). El primer caràcter ha de ser a-z o A-Z. La longitut ha d'estar entre 6 i 16 caràcters."),
        ("Website", "Lloc web"),
        ("About", "Sobre"),
        ("Slogan_tip", ""),
        ("Privacy Statement", "Declaració de privacitat"),
        ("Mute", "Silencia"),
        ("Build Date", "Data de creació"),
        ("Version", "Versió"),
        ("Home", "Inici"),
        ("Audio Input", "Entrada d'àudio"),
        ("Enhancements", "Millores"),
        ("Hardware Codec", "Còdec de hardware"),
        ("Adaptive bitrate", "Tasa de bits adaptativa"),
        ("ID Server", "Servidor de IDs"),
        ("Relay Server", "Servidor Relay"),
        ("API Server", "Servidor API"),
        ("invalid_http", "ha de començar amb http:// o https://"),
        ("Invalid IP", "IP incorrecta"),
        ("Invalid format", "Format incorrecte"),
        ("server_not_support", "Encara no és compatible amb el servidor"),
        ("Not available", "No disponible"),
        ("Too frequent", "Massa comú"),
        ("Cancel", "Cancel·la"),
        ("Skip", "Salta"),
        ("Close", "Tanca"),
        ("Retry", "Reintenta"),
        ("OK", "D'acord"),
        ("Password Required", "Es necessita la contrasenya"),
        ("Please enter your password", "Introdueix la teva contrasenya"),
        ("Remember password", "Recorda la contrasenya"),
        ("Wrong Password", "Contrasenya incorrecta"),
        ("Do you want to enter again?", "Vols tornar a entrar?"),
        ("Connection Error", "Error de connexió"),
        ("Error", "Error"),
        ("Reset by the peer", "Reestablert pel peer"),
        ("Connecting...", "Connectant..."),
        ("Connection in progress. Please wait.", "Connexió en procés. Espera."),
        ("Please try 1 minute later", "Torna a provar-ho d'aquí un minut"),
        ("Login Error", "Error d'inici de sessió"),
        ("Successful", "Exitós"),
        ("Connected, waiting for image...", "Connectant, esperant imatge..."),
        ("Name", "Nom"),
        ("Type", "Tipus"),
        ("Modified", "Modificat"),
        ("Size", "Grandària"),
        ("Show Hidden Files", "Mostra arxius ocults"),
        ("Receive", "Rep"),
        ("Send", "Envia"),
        ("Refresh File", "Actualitza el fitxer"),
        ("Local", "Local"),
        ("Remote", "Remot"),
        ("Remote Computer", "Ordinador remot"),
        ("Local Computer", "Ordinador local"),
        ("Confirm Delete", "Confirma l'eliminació"),
        ("Delete", "Elimina"),
        ("Properties", "Propietats"),
        ("Multi Select", "Selecció múltiple"),
        ("Select All", "Selecciona-ho tot"),
        ("Unselect All", "Deselecciona-ho tot"),
        ("Empty Directory", "Carpeta buida"),
        ("Not an empty directory", "No és una carpeta buida"),
        ("Are you sure you want to delete this file?", "Segur que vols eliminar aquest fitxer?"),
        ("Are you sure you want to delete this empty directory?", "Segur que vols eliminar aquesta carpeta buida?"),
        ("Are you sure you want to delete the file of this directory?", "Segur que vols eliminar aquest fitxer d'aquesta car`peta?"),
        ("Do this for all conflicts", "Fes això per a tots els conflictes"),
        ("This is irreversible!", "Això és irreversible!"),
        ("Deleting", "Eliminant"),
        ("files", "fitxers"),
        ("Waiting", "Esperant"),
        ("Finished", "Acabat"),
        ("Speed", "Velocitat"),
        ("Custom Image Quality", "Qualitat d'imatge personalitzada"),
        ("Privacy mode", "Mode privat"),
        ("Block user input", "Bloqueja l'entrada d'usuari"),
        ("Unblock user input", "Desbloqueja l'entrada d'usuari"),
        ("Adjust Window", "Ajusta la finestra"),
        ("Original", "Original"),
        ("Shrink", "Reduir"),
        ("Stretch", "Estirar"),
        ("Scrollbar", "Barra de desplaçament"),
        ("ScrollAuto", "Desplaçament automàtic"),
        ("Good image quality", "Bona qualitat d'imatge"),
        ("Balanced", "Equilibrat"),
        ("Optimize reaction time", "Optimitza el temps de reacció"),
        ("Custom", "Personalitzat"),
        ("Show remote cursor", "Mostra el cursor remot"),
        ("Show quality monitor", "Mostra la qualitat del monitor"),
        ("Disable clipboard", "Deshabilita el portapapers"),
        ("Lock after session end", "Bloqueja després del final de la sessió"),
        ("Insert", "Insereix"),
        ("Insert Lock", "Insereix bloqueig"),
        ("Refresh", "Actualitza"),
        ("ID does not exist", "L'ID no existeix"),
        ("Failed to connect to rendezvous server", "No es pot connectar al servidor rendezvous"),
        ("Please try later", "Prova-ho més tard"),
        ("Remote desktop is offline", "L'escriptori remot està desconecctat"),
        ("Key mismatch", "La clau no coincideix"),
        ("Timeout", "Temps esgotat"),
        ("Failed to connect to relay server", "No es pot connectar al servidor de relay"),
        ("Failed to connect via rendezvous server", "No es pot connectar a través del servidor de rendezvous"),
        ("Failed to connect via relay server", "No es pot connectar a través del servidor de relay"),
        ("Failed to make direct connection to remote desktop", "No s'ha pogut establir una connexió directa amb l'escriptori remot"),
        ("Set Password", "Configura la contrasenya"),
        ("OS Password", "contrasenya del sistema operatiu"),
        ("install_tip", ""),
        ("Click to upgrade", "Clica per a actualitzar"),
        ("Click to download", "Clica per a descarregar"),
        ("Click to update", "Clica per a refrescar"),
        ("Configure", "Configurr"),
        ("config_acc", ""),
        ("config_screen", "Configurar pantalla"),
        ("Installing ...", "Instal·lant ..."),
        ("Install", "Instal·la"),
        ("Installation", "Instal·lació"),
        ("Installation Path", "Ruta d'instal·lació"),
        ("Create start menu shortcuts", "Crear accessos directes al menú d'inici"),
        ("Create desktop icon", "Crear icona d'escriptori"),
        ("agreement_tip", ""),
        ("Accept and Install", "Acceptar i instal·la"),
        ("End-user license agreement", "Acord de llicència d'usuari final"),
        ("Generating ...", "Generant ..."),
        ("Your installation is lower version.", "La teva instal·lació és una versión inferior."),
        ("not_close_tcp_tip", ""),
        ("Listening ...", "Escoltant..."),
        ("Remote Host", "Hoste remot"),
        ("Remote Port", "Port remot"),
        ("Action", "Acció"),
        ("Add", "Afegeix"),
        ("Local Port", "Port local"),
        ("Local Address", "Adreça Local"),
        ("Change Local Port", "Canvia el port local"),
        ("setup_server_tip", ""),
        ("Too short, at least 6 characters.", "Massa curt, almenys 6 caràcters."),
        ("The confirmation is not identical.", "La confirmació no coincideix."),
        ("Permissions", "Permisos"),
        ("Accept", "Accepta"),
        ("Dismiss", "Cancel·la"),
        ("Disconnect", "Desconnecta"),
        ("Enable file copy and paste", "Permet copiar i enganxar fitxers"),
        ("Connected", "Connectat"),
        ("Direct and encrypted connection", "Connexió directa i xifrada"),
        ("Relayed and encrypted connection", "Connexió retransmesa i xifrada"),
        ("Direct and unencrypted connection", "Connexió directa i sense xifrar"),
        ("Relayed and unencrypted connection", "Connexió retransmesa i sense xifrar"),
        ("Enter Remote ID", "Introdueix l'ID remot"),
        ("Enter your password", "Introdueix la teva contrasenya"),
        ("Logging in...", "Iniciant sessió..."),
        ("Enable RDP session sharing", "Habilita l'ús compartit de sessions RDP"),
        ("Auto Login", "Inici de sessió automàtic"),
        ("Enable direct IP access", "Habilita accés IP directe"),
        ("Rename", "Renombra"),
        ("Space", "Espai"),
        ("Create desktop shortcut", "Crea accés directe a l'escriptori"),
        ("Change Path", "Canvia la ruta"),
        ("Create Folder", "Crea carpeta"),
        ("Please enter the folder name", "Indica el nom de la carpeta"),
        ("Fix it", "Soluciona-ho"),
        ("Warning", "Avís"),
        ("Login screen using Wayland is not supported", "La pantalla d'inici de sessió amb Wayland no és compatible"),
        ("Reboot required", "Cal reiniciar"),
        ("Unsupported display server", "Servidor de visualització no compatible"),
        ("x11 expected", "x11 necessari"),
        ("Port", "Port"),
        ("Settings", "Ajustaments"),
        ("Username", " Nom d'usuari"),
        ("Invalid port", "Port incorrecte"),
        ("Closed manually by the peer", "Tancat manualment pel peer"),
        ("Enable remote configuration modification", "Habilitar modificació remota de configuració"),
        ("Run without install", "Executa sense instal·lar"),
        ("Connect via relay", "Connecta per relay"),
        ("Always connect via relay", "Connecta sempre a través de relay"),
        ("whitelist_tip", ""),
        ("Login", "Inicia sessió"),
        ("Verify", "Verifica"),
        ("Remember me", "Recorda'm"),
        ("Trust this device", "Confia en aquest dispositiu"),
        ("Verification code", "Codi de verificació"),
        ("verification_tip", ""),
        ("Logout", "Sortir"),
        ("Tags", "Etiquetes"),
        ("Search ID", "Cerca ID"),
        ("whitelist_sep", ""),
        ("Add ID", "Afegir ID"),
        ("Add Tag", "Afegir tag"),
        ("Unselect all tags", "Deseleccionar tots els tags"),
        ("Network error", "Error de xarxa"),
        ("Username missed", "Nom d'usuari oblidat"),
        ("Password missed", "Contrasenya oblidada"),
        ("Wrong credentials", "Credencials incorrectes"),
        ("The verification code is incorrect or has expired", "El codi de verificació es incorrecte o ha expirat"),
        ("Edit Tag", "Editar tag"),
        ("Forget Password", "Contrasenya oblidada"),
        ("Favorites", "Preferits"),
        ("Add to Favorites", "Afegir a preferits"),
        ("Remove from Favorites", "Treure de preferits"),
        ("Empty", "Buit"),
        ("Invalid folder name", "Nom de carpeta incorrecte"),
        ("Socks5 Proxy", "Proxy Socks5"),
        ("Socks5/Http(s) Proxy", "Proxy Socks5/Http(s)"),
        ("Discovered", "Descobert"),
        ("install_daemon_tip", ""),
        ("Remote ID", "ID remot"),
        ("Paste", "Enganxa"),
        ("Paste here?", "Enganxa-ho aquí?"),
        ("Are you sure to close the connection?", "Segur que vols tancar la connexió?"),
        ("Download new version", "Descarrega una nova versió"),
        ("Touch mode", "Mode tàctil"),
        ("Mouse mode", "Mode ratolí"),
        ("One-Finger Tap", "Toca amb un dit"),
        ("Left Mouse", "Ratolí esquerra"),
        ("One-Long Tap", "Toc llarg"),
        ("Two-Finger Tap", "Toqui amb dos dits"),
        ("Right Mouse", "Botó dret"),
        ("One-Finger Move", "Moviment amb un dir"),
        ("Double Tap & Move", "Toca dos cops i mogui"),
        ("Mouse Drag", "Arrossega amb el ratolí"),
        ("Three-Finger vertically", "Tres dits verticalment"),
        ("Mouse Wheel", "Roda del ratolí"),
        ("Two-Finger Move", "Moviment amb dos dits"),
        ("Canvas Move", "Moviment del llenç"),
        ("Pinch to Zoom", "Pessiga per fer zoom"),
        ("Canvas Zoom", "Amplia el llenç"),
        ("Reset canvas", "Reestableix el llenç"),
        ("No permission of file transfer", "No tens permís de transferència de fitxers"),
        ("Note", "Nota"),
        ("Connection", "Connexió"),
        ("Share Screen", "Comparteix la pantalla"),
        ("Chat", "Xat"),
        ("Total", "Total"),
        ("items", "ítems"),
        ("Selected", "Seleccionat"),
        ("Screen Capture", "Captura de pantalla"),
        ("Input Control", "Control d'entrada"),
        ("Audio Capture", "Captura d'àudio"),
        ("File Connection", "Connexió d'arxius"),
        ("Screen Connection", "Connexió de pantalla"),
        ("Do you accept?", "Acceptes?"),
        ("Open System Setting", "Configuració del sistema obert"),
        ("How to get Android input permission?", "Com obtenir el permís d'entrada d'Android?"),
        ("android_input_permission_tip1", "Per a que un dispositiu remot controli el seu dispositiu Android amb el ratolí o tocs, cal permetre que RustDesk utilitzi el servei d' \"Accesibilitat\"."),
        ("android_input_permission_tip2", "Vés a la pàgina de [Serveis instal·lats], activa el servei [RustDesk Input]."),
        ("android_new_connection_tip", "S'ha rebut una nova sol·licitud de control per al dispositiu actual."),
        ("android_service_will_start_tip", "Habilitar la captura de pantalla iniciarà el servei automàticament, i permetrà que altres dispositius sol·licitin una connexió des d'aquest dispositiu."),
        ("android_stop_service_tip", "Tancar el servei tancarà totes les connexions establertes."),
        ("android_version_audio_tip", "La versión actual de Android no admet la captura d'àudio, actualizi a Android 10 o superior."),
        ("android_start_service_tip", ""),
        ("android_permission_may_not_change_tip", ""),
        ("Account", "Compte"),
        ("Overwrite", "Sobreescriu"),
        ("This file exists, skip or overwrite this file?", "Aquest fitxer ja existeix, ometre o sobreescriure l'arxiu?"),
        ("Quit", "Surt"),
        ("Help", "Ajuda"),
        ("Failed", "Ha fallat"),
        ("Succeeded", "Aconseguit"),
        ("Someone turns on privacy mode, exit", "Algú ha activat el mode de privacitat, surt"),
        ("Unsupported", "No suportat"),
        ("Peer denied", "Peer denegat"),
        ("Please install plugins", "Instal·la els complements"),
        ("Peer exit", "El peer ha sortit"),
        ("Failed to turn off", "Error en apagar"),
        ("Turned off", "Apagat"),
        ("Language", "Idioma"),
        ("Keep RustDesk background service", "Mantenir RustDesk com a servei en segon pla"),
        ("Ignore Battery Optimizations", "Ignora optimizacions de la bateria"),
        ("android_open_battery_optimizations_tip", ""),
        ("Start on boot", "Engega en l'arrencada"),
        ("Start the screen sharing service on boot, requires special permissions", "Engega el servei de captura de pantalla en l'arrencada, requereix permisos especials"),
        ("Connection not allowed", "Connexió no disponible"),
        ("Legacy mode", "Mode heretat"),
        ("Map mode", "Mode mapa"),
        ("Translate mode", "Mode traduit"),
        ("Use permanent password", "Utilitza una contrasenya permament"),
        ("Use both passwords", "Utilitza ambdues contrasenyas"),
        ("Set permanent password", "Estableix una contrasenya permament"),
        ("Enable remote restart", "Activa el reinici remot"),
        ("Restart remote device", "Reinicia el dispositiu"),
        ("Are you sure you want to restart", "Segur que vol reiniciar?"),
        ("Restarting remote device", "Reiniciant el dispositiu remot"),
        ("remote_restarting_tip", "Reiniciant el dispositiu remot, tanca aquest missatge i torna't a connectar amb la contrasenya."),
        ("Copied", "Copiat"),
        ("Exit Fullscreen", "Surt de la pantalla completa"),
        ("Fullscreen", "Pantalla completa"),
        ("Mobile Actions", "Accions mòbils"),
        ("Select Monitor", "Selecciona el monitor"),
        ("Control Actions", "Accions de control"),
        ("Display Settings", "Configuració de pantalla"),
        ("Ratio", "Relació"),
        ("Image Quality", "Qualitat d'imatge"),
        ("Scroll Style", "Estil de desplaçament"),
        ("Show Toolbar", "Mostra la barra d'eines"),
        ("Hide Toolbar", "Amaga la barra d'eines"),
        ("Direct Connection", "Connexió directa"),
        ("Relay Connection", "Connexió Relay"),
        ("Secure Connection", "Connexió segura"),
        ("Insecure Connection", "Connexió insegura"),
        ("Scale original", "Escala original"),
        ("Scale adaptive", "Escala adaptativa"),
        ("General", "General"),
        ("Security", "Seguretat"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tema Fosc"),
        ("Light Theme", "Tema clar"),
        ("Dark", "Fosc"),
        ("Light", "Clar"),
        ("Follow System", "Tema del sistema"),
        ("Enable hardware codec", "Habilita el còdec per hardware"),
        ("Unlock Security Settings", "Desbloqueja els ajustaments de seguretat"),
        ("Enable audio", "Habilita l'àudio"),
        ("Unlock Network Settings", "Desbloqueja els ajustaments de xarxa"),
        ("Server", "Servidor"),
        ("Direct IP Access", "Accés IP Directe"),
        ("Proxy", "Proxy"),
        ("Apply", "Aplica"),
        ("Disconnect all devices?", "Vols desconnectar tots els dispositius?"),
        ("Clear", "Neteja"),
        ("Audio Input Device", "Dispositiu d'entrada d'àudio"),
        ("Use IP Whitelisting", "Utilitza llista de IPs admeses"),
        ("Network", "Xarxa"),
        ("Pin Toolbar", "Fixa la barra d'eines"),
        ("Unpin Toolbar", "Deixa de fixar la barra d'eines"),
        ("Recording", "Gravant"),
        ("Directory", "Directori"),
        ("Automatically record incoming sessions", "Gravació automàtica de sessions entrants"),
        ("Change", "Canvia"),
        ("Start session recording", "Comença la gravació de la sessió"),
        ("Stop session recording", "Atura la gravació de la sessió"),
        ("Enable recording session", "Habilita la gravació de la sessió"),
        ("Enable LAN discovery", "Habilita el descobriment de LAN"),
        ("Deny LAN discovery", "Denega el descobriment de LAN"),
        ("Write a message", "Escriu un missatge"),
        ("Prompt", "Consulta"),
        ("Please wait for confirmation of UAC...", "Espera per confirmar l'UAC..."),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", "Desconnectat"),
        ("Other", "Altre"),
        ("Confirm before closing multiple tabs", "Confirma abans de tancar múltiples pestanyes"),
        ("Keyboard Settings", "Ajustaments de teclat"),
        ("Full Access", "Acces complet"),
        ("Screen Share", "Comparteix la pantalla"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland requereix Ubuntu 21.04 o una versió superior."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland requereix una versió superior de la distribución de Linux. Prova l'escriptori X11 o canvia el teu sistema operatiu."),
        ("JumpLink", "Veure"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Selecciona la pantalla que es compartirà (Opera al costat del peer)."),
        ("Show RustDesk", "Mostra RustDesk"),
        ("This PC", "Aquest PC"),
        ("or", "o"),
        ("Continue with", "Continur amb"),
        ("Elevate", "Eleva"),
        ("Zoom cursor", "Zoom del ratolí"),
        ("Accept sessions via password", "Accepta sessions via contrasenya"),
        ("Accept sessions via click", "Accepta sessions via clic"),
        ("Accept sessions via both", "Accepta sessions via les dues opcions"),
        ("Please wait for the remote side to accept your session request...", "Esperea que la part remota accepti la teva sol·licitud de sessió..."),
        ("One-time Password", "Contrasenya d'un sol ús"),
        ("Use one-time password", "Fes servir una contrasenya d'un sol ús"),
        ("One-time password length", "Caracters de la contrasenya d'un sol ús"),
        ("Request access to your device", "Sol·licita l'acces al teu dispositiu"),
        ("Hide connection management window", "Amaga la finestra de gestió de connexió"),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", "Clic dret per seleccionar les pestanyes"),
        ("Skipped", "Saltat"),
        ("Add to address book", "Afegeix a la llibreta d'adreces"),
        ("Group", "Grup"),
        ("Search", "Cerca"),
        ("Closed manually by web console", "Tancat manualment amb la consola web"),
        ("Local keyboard type", "Tipus de teclat local"),
        ("Select local keyboard type", "Selecciona el tipus de teclat local"),
        ("software_render_tip", ""),
        ("Always use software rendering", "Fes servir sempre la renderització per software"),
        ("config_input", ""),
        ("config_microphone", ""),
        ("request_elevation_tip", ""),
        ("Wait", "Espera"),
        ("Elevation Error", "Error d'elevació"),
        ("Ask the remote user for authentication", "Demana autenticació a l'usuari remot"),
        ("Choose this if the remote account is administrator", "Selecciona això si l'usuari remot és administrador"),
        ("Transmit the username and password of administrator", "Transmet el nom d'usuari i la contrasenya de l'administrador"),
        ("still_click_uac_tip", ""),
        ("Request Elevation", "Demana l'elevació"),
        ("wait_accept_uac_tip", ""),
        ("Elevate successfully", "Elevació exitosa"),
        ("uppercase", "majúscula"),
        ("lowercase", "minúscula"),
        ("digit", "dígit"),
        ("special character", "caràcter especial"),
        ("length>=8", "longitut>=8"),
        ("Weak", "Dèbil"),
        ("Medium", "Mitja"),
        ("Strong", "Forta"),
        ("Switch Sides", "Canvia de costat"),
        ("Please confirm if you want to share your desktop?", "Confirma que vols compartir el teu escriptori?"),
        ("Display", "Pantalla"),
        ("Default View Style", "Estil de visualització predeterminat"),
        ("Default Scroll Style", "Estil de desplaçament predeterminat"),
        ("Default Image Quality", "Qualitat d'imatge predeterminada"),
        ("Default Codec", "Còdec predeterminat"),
        ("Bitrate", "Ratio de bits"),
        ("FPS", "FPS"),
        ("Auto", "Auto"),
        ("Other Default Options", "Altres opcions predeterminades"),
        ("Voice call", "Trucada de veu"),
        ("Text chat", "Xat de text"),
        ("Stop voice call", "Penja la trucada de veu"),
        ("relay_hint_tip", ""),
        ("Reconnect", "Reconecta"),
        ("Codec", "Còdec"),
        ("Resolution", "Resolució"),
        ("No transfers in progress", "Sense transferències en curs"),
        ("Set one-time password length", "Selecciona la longitud de la contrasenya d'un sol ús"),
        ("RDP Settings", "Configuració RDP"),
        ("Sort by", "Ordena per"),
        ("New Connection", "Nova connexió"),
        ("Restore", "Restaura"),
        ("Minimize", "Minimitza"),
        ("Maximize", "Maximtiza"),
        ("Your Device", "El teu dispositiu"),
        ("empty_recent_tip", ""),
        ("empty_favorite_tip", ""),
        ("empty_lan_tip", ""),
        ("empty_address_book_tip", ""),
        ("eg: admin", "p.ex.: admin"),
        ("Empty Username", "Usuari buit"),
        ("Empty Password", "Contrasenya buida"),
        ("Me", "Jo"),
        ("identical_file_tip", ""),
        ("show_monitors_tip", ""),
        ("View Mode", "Tipus de visualització"),
        ("login_linux_tip", ""),
        ("verify_rustdesk_password_tip", ""),
        ("remember_account_tip", ""),
        ("os_account_desk_tip", ""),
        ("OS Account", ""),
        ("another_user_login_title_tip", ""),
        ("another_user_login_text_tip", ""),
        ("xorg_not_found_title_tip", ""),
        ("xorg_not_found_text_tip", ""),
        ("no_desktop_title_tip", ""),
        ("no_desktop_text_tip", ""),
        ("No need to elevate", "No cal elevar permisos"),
        ("System Sound", "Sistema de so"),
        ("Default", "Predeterminat"),
        ("New RDP", "Nou RDP"),
        ("Fingerprint", "Empremta digital"),
        ("Copy Fingerprint", "Copia l'emprenta digital"),
        ("no fingerprints", "sense emprentes"),
        ("Select a peer", "Selecciona un peer"),
        ("Select peers", "Selecciona diversos peers"),
        ("Plugins", "Complements"),
        ("Uninstall", "Desinstal·la"),
        ("Update", "Actualitza"),
        ("Enable", "Activa"),
        ("Disable", "Desactiva"),
        ("Options", "Opcions"),
        ("resolution_original_tip", ""),
        ("resolution_fit_local_tip", ""),
        ("resolution_custom_tip", ""),
        ("Collapse toolbar", "Col·lapsa la barra d'etiquetes"),
        ("Accept and Elevate", "Accepta i eleva"),
        ("accept_and_elevate_btn_tooltip", ""),
        ("clipboard_wait_response_timeout_tip", ""),
        ("Incoming connection", "Connexió entrant"),
        ("Outgoing connection", "Connexió sortint"),
        ("Exit", "Tanca"),
        ("Open", "Obre"),
        ("logout_tip", ""),
        ("Service", "Servei"),
        ("Start", "Inicia"),
        ("Stop", "Atura"),
        ("exceed_max_devices", ""),
        ("Sync with recent sessions", "Sincronitza amb les sessions recents"),
        ("Sort tags", "Ordena per etiquetes"),
        ("Open connection in new tab", "Obre la connexió en una nova pestanya"),
        ("Move tab to new window", "Mou la pestanya a una nova finestra"),
        ("Can not be empty", "No pot estar buit"),
        ("Already exists", "Ja existeix"),
        ("Change Password", "Canvia la contrasenya"),
        ("Refresh Password", "Refresca la contrasenya"),
        ("ID", "ID"),
        ("Grid View", "Visualització de graella"),
        ("List View", "Visualització de llista"),
        ("Select", "Selecciona"),
        ("Toggle Tags", "Activa/desactiva les etiquetes"),
        ("pull_ab_failed_tip", ""),
        ("push_ab_failed_tip", ""),
        ("synced_peer_readded_tip", ""),
        ("Change Color", "Canvia el color"),
        ("Primary Color", "Color primari"),
        ("HSV Color", "Color HSV"),
        ("Installation Successful!", "Instal·lació correcta!"),
        ("Installation failed!", "Ha fallat la instal·lació!"),
        ("Reverse mouse wheel", "Canvia l'orientació de la roda del ratolí"),
        ("{} sessions", "{} sessions"),
        ("scam_title", ""),
        ("scam_text1", ""),
        ("scam_text2", ""),
        ("Don't show again", "No ho mostris més"),
        ("I Agree", "Accepta"),
        ("Decline", "Rebutja"),
        ("Timeout in minutes", "Desconexió en minuts"),
        ("auto_disconnect_option_tip", ""),
        ("Connection failed due to inactivity", "Connexió fallada per inactivitat"),
        ("Check for software update on startup", "Revisa les actualitzacions de software en iniciar"),
        ("upgrade_rustdesk_server_pro_to_{}_tip", ""),
        ("pull_group_failed_tip", ""),
        ("Filter by intersection", "Filtra per intersecció"),
        ("Remove wallpaper during incoming sessions", "Amaga el fons de pantalla en les connexions entrants"),
        ("Test", "Prova"),
        ("display_is_plugged_out_msg", ""),
        ("No displays", "Sense pantalles"),
        ("Open in new window", "Obre en una nova finestra"),
        ("Show displays as individual windows", "Mostra les pantalles com finestres individuals"),
        ("Use all my displays for the remote session", "Fes servir totes les meves pantalles per la sessió remota"),
        ("selinux_tip", ""),
        ("Change view", "Canvia la vista"),
        ("Big tiles", "Títols grans"),
        ("Small tiles", "Títols petits"),
        ("List", "Llista"),
        ("Virtual display", "Pantalla virtual"),
        ("Plug out all", "Desconnectar tots"),
        ("True color (4:4:4)", "Color real (4:4:4)"),
        ("Enable blocking user input", "Activa el bloqueig d'entrada d'usuari"),
        ("id_input_tip", ""),
        ("privacy_mode_impl_mag_tip", ""),
        ("privacy_mode_impl_virtual_display_tip", ""),
        ("Enter privacy mode", "Entra al mode de privacitat"),
        ("Exit privacy mode", "Surt del mode de privacitat"),
        ("idd_not_support_under_win10_2004_tip", ""),
        ("input_source_1_tip", ""),
        ("input_source_2_tip", ""),
        ("Swap control-command key", "Canvia la tecla de control"),
        ("swap-left-right-mouse", ""),
        ("2FA code", "Codi 2FA"),
        ("More", "Més"),
        ("enable-2fa-title", ""),
        ("enable-2fa-desc", ""),
        ("wrong-2fa-code", ""),
        ("enter-2fa-title", ""),
        ("Email verification code must be 6 characters.", "El codi de verificació de l'email ha de tenir 6 caràcters."),
        ("2FA code must be 6 digits.", "El codi 2FA ha de tenir 6 digits."),
        ("Multiple Windows sessions found", "Multiples sessions de Windows trobades"),
        ("Please select the session you want to connect to", "Selecciona la sessió a la què et vols connectar"),
        ("powered_by_me", ""),
        ("outgoing_only_desk_tip", ""),
        ("preset_password_warning", ""),
        ("Security Alert", "Alerta de seguretat"),
        ("My address book", "La meva llibreta d'adreces"),
        ("Personal", "Personal"),
        ("Owner", "Propietari"),
        ("Set shared password", "Establir la contrasenya compartida"),
        ("Exist in", "Existeix en"),
        ("Read-only", "Només lectura"),
        ("Read/Write", "Lectura/Escriptura"),
        ("Full Control", "Control total"),
        ("share_warning_tip", ""),
        ("Everyone", "Tots"),
        ("ab_web_console_tip", ""),
        ("allow-only-conn-window-open-tip", ""),
        ("no_need_privacy_mode_no_physical_displays_tip", ""),
        ("Follow remote cursor", "Segueix el cursor remot"),
        ("Follow remote window focus", "Segueix el focus de la finestra remota"),
        ("default_proxy_tip", ""),
        ("no_audio_input_device_tip", ""),
        ("Incoming", "Entrant"),
        ("Outgoing", "Sortint"),
        ("Clear Wayland screen selection", "Neteja la selecció de pantalla Wayland"),
        ("clear_Wayland_screen_selection_tip", ""),
        ("confirm_clear_Wayland_screen_selection_tip", ""),
        ("android_new_voice_call_tip", ""),
        ("texture_render_tip", ""),
        ("Use texture rendering", "Fes servir la renderització de textures"),
        ("Floating window", "Finestra flotant"),
        ("floating_window_tip", ""),
        ("Keep screen on", "Deixa la pantalla encesa"),
        ("Never", "Mai"),
        ("During controlled", "Mentre estigui controlat"),
        ("During service is on", "Mentre el servei estigui encés"),
        ("Capture screen using DirectX", "Captura de pantalla utilitzant DirectX"),
        ("Back", "Enrere"),
        ("Apps", "Aplicacions"),
        ("Volume up", "Puja el volum"),
        ("Volume down", "Baixa el volum"),
        ("Power", "Engega"),
        ("Telegram bot", "Bot de Telegram"),
        ("enable-bot-tip", ""),
        ("enable-bot-desc", ""),
        ("cancel-2fa-confirm-tip", ""),
        ("cancel-bot-confirm-tip", ""),
        ("About RustDesk", ""),
        ("Send clipboard keystrokes", ""),
        ("network_error_tip", ""),
        ("Unlock with PIN", ""),
        ("Requires at least {} characters", ""),
        ("Wrong PIN", ""),
        ("Set PIN", ""),
        ("Enable trusted devices", ""),
        ("Manage trusted devices", ""),
        ("Platform", ""),
        ("Days remaining", ""),
        ("enable-trusted-devices-tip", ""),
        ("Parent directory", ""),
        ("Resume", ""),
        ("Invalid file name", ""),
    ].iter().cloned().collect();
}