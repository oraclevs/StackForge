use anyhow::{Result, anyhow};
use serde_yaml::{Mapping, Value};
use std::fs;
use std::path::Path;
pub struct FlutterFiles {
    name: String,
}

impl FlutterFiles {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    fn app_routes(&self) -> String {
        format!(
            r#"
import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:{name}/app/navigation.dart';
import 'package:{name}/core/services/cache.dart';
import 'package:{name}/shared/screens/home/index.dart';

final hasOnboarded = ValueNotifier<bool?>(null);

Future<void> loadOnboardingState() async {{
  final db = CacheService();
  hasOnboarded.value = await db.getOnboardingState();
    }}

final routerProvider = Provider(
  (ref) => GoRouter(
    initialLocation: AppRoutesNames.homeIndex,
    // refreshListenable: hasOnboarded,
    // debugLogDiagnostics: true,
    routes: [
      AnimatedGoRoute(
        path: AppRoutesNames.homeIndex,
        name: AppRoutesNames.homeIndex,
        builder: (context, state) => const HomeIndex(),
      ),
    ],
  ),
);

class AnimatedGoRoute extends GoRoute {{
  AnimatedGoRoute({{
    required super.path,
    super.name,
    Widget Function(BuildContext, GoRouterState)? builder,
    Page<dynamic> Function(BuildContext, GoRouterState)? pageBuilder,
    super.parentNavigatorKey,
    super.redirect,
    super.onExit,
    super.caseSensitive,
    super.routes,
    }}) : super(
         pageBuilder:
             pageBuilder ??
             (builder == null
                 ? null
                 : (context, state) => _randomTransitionPage(
                     state: state,
                     child: builder(context, state),
                   )),

         builder: pageBuilder == null ? null : builder,
       );

  static CustomTransitionPage _randomTransitionPage({{
    required GoRouterState state,
    required Widget child,
        }}) {{
    final random = Random();
    final type = random.nextInt(4);

    return CustomTransitionPage(
      key: state.pageKey,
      transitionDuration: const Duration(milliseconds: 400),
      child: child,
      transitionsBuilder: (context, animation, secondaryAnimation, child) {{
        switch (type) {{
          // Fade
          case 0:
            return FadeTransition(opacity: animation, child: child);
          // Slide from right
          case 1:
            return SlideTransition(
              position:
                  Tween<Offset>(
                    begin: const Offset(1, 0),
                    end: Offset.zero,
                  ).animate(
                    CurvedAnimation(
                      parent: animation,
                      curve: Curves.easeOutCubic,
                    ),
                  ),
              child: child,
            );

          // Scale
          case 2:
            return ScaleTransition(
              scale: Tween<double>(begin: 0.9, end: 1.0).animate(
                CurvedAnimation(parent: animation, curve: Curves.easeOutBack),
              ),
              child: child,
            );

          // Rotation + Fade
          default:
            return RotationTransition(
              turns: Tween<double>(begin: -0.04, end: 0.0).animate(animation),
              child: FadeTransition(opacity: animation, child: child),
            );
    }}
    }},
    );
    }}
    }}

        "#,
            name = self.name
        )
    }
    fn app(&self) -> String {
        format!(
            r#"
        
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:{}/core/themes/theme.dart';

import 'app_routes.dart';

class App extends ConsumerStatefulWidget {{
  const App({{super.key}});

  @override
  ConsumerState<App> createState() => _HandiState();
    }}

class _HandiState extends ConsumerState<App> {{
  @override
  Widget build(BuildContext context) {{
    final router = ref.watch(routerProvider);
    return MaterialApp.router(
      title: 'App',
      theme: AppTheme.light(),
      themeMode: ThemeMode.light,
      debugShowCheckedModeBanner: false,
      routerConfig: router,
      builder: (context, child) {{
        final mediaQuery = MediaQuery.of(context);
        return MediaQuery(
          data: mediaQuery.copyWith(
            viewInsets: EdgeInsets.zero,
            textScaler: const TextScaler.linear(1.0),
          ),
          child: child!,
        );
    }},
    );
    }}
    }}

        "#,
            self.name
        )
    }
    fn navigation(&self) -> String {
        r#"

/// This is the class the contained all the path names for [GoRouter]
class AppRoutesNames {
  const AppRoutesNames._();
  //Onboarding screen
  static const String onboardingIndex = '/';
  static const String homeIndex = '/home';
}

        "#
        .to_string()
    }
    fn api_base_class(&self) -> String {
        format!(
            r#"
import 'dart:async';
import 'dart:math';
import 'package:dio/dio.dart';
import 'package:flutter/foundation.dart';
import 'package:{}/core/services/api_constant.dart';


/// Standard API response wrapper
class ApiResponse<T> {{
  final bool success;
  final T? data;
  final int? statusCode;
  final String message;
  final DioException? dioError;

  ApiResponse.success(this.data, this.message, {{this.statusCode}})
    : success = true,
      dioError = null;

  ApiResponse.error(this.message, {{this.statusCode, this.dioError}})
    : success = false,
      data = null;

  @override
  String toString() =>
      'ApiResponse(success: $success, statusCode: $statusCode, message: $message, data: $data)';
    }}

typedef UnauthorizedCallback = FutureOr<void> Function();

/// Api base service class with all the CRUD methods
class ApiServiceBase {{
  ApiServiceBase({{
    String? baseUrl,
    Duration? connectTimeout,
    Duration? receiveTimeout,
    this.onUnauthorized,
    this.maxRetries = 2,
    }}) {{
    base = baseUrl ?? ApiConstants.baseEndpoint;
    options = BaseOptions(
      baseUrl: base,
      connectTimeout: connectTimeout ?? const Duration(seconds: 60),
      receiveTimeout: receiveTimeout ?? const Duration(seconds: 60),
      responseType: ResponseType.json,
      headers: {{
        'Accept': 'application/json',
        'Content-Type': 'application/json',
    }},
    );
    dio = Dio(options);
    dio.interceptors.add(
      LogInterceptor(
        request: true,
        requestHeader: true,
        requestBody: true,
        responseHeader: true,
        responseBody: true,
        error: true,
        logPrint: (obj) => debugPrint('[ApiServiceBase]:::$obj'),
      ),
    );
    }}

  late String base;
  late final BaseOptions options;
  late final Dio dio;

  /// Optional callback used when 401 encountered
  final UnauthorizedCallback? onUnauthorized;

  /// How many times to retry for transient errors (network/429). 0 => no retry
  final int maxRetries;

  /// Store auth token locally; used to set Authorization header on all requests
  String? _authToken;
  void setAuthToken(String? token) {{
    _authToken = token;
    if (token != null) {{
      dio.options.headers['Authorization'] = 'Bearer $token';
    }} else {{
      dio.options.headers.remove('Authorization');
    }}
  }}

  // --------------------------
  // Public convenience methods
  // --------------------------
  Future<ApiResponse<T>> get<T>(
    String path, {{
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) => _request<T>(
    path,
    method: 'GET',
    queryParameters: queryParameters,
    options: options,
    cancelToken: cancelToken,
    forceRetries: forceRetries,
  );

  Future<ApiResponse<T>> post<T>(
    String path, {{
    Object? data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) => _request<T>(
    path,
    method: 'POST',
    data: data,
    queryParameters: queryParameters,
    options: options,
    cancelToken: cancelToken,
    forceRetries: forceRetries,
  );

  Future<ApiResponse<T>> put<T>(
    String path, {{
    Object? data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) => _request<T>(
    path,
    method: 'PUT',
    data: data,
    queryParameters: queryParameters,
    options: options,
    cancelToken: cancelToken,
    forceRetries: forceRetries,
  );

  Future<ApiResponse<T>> patch<T>(
    String path, {{
    Object? data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) => _request<T>(
    path,
    method: 'PATCH',
    data: data,
    queryParameters: queryParameters,
    options: options,
    cancelToken: cancelToken,
    forceRetries: forceRetries,
  );

  Future<ApiResponse<T>> delete<T>(
    String path, {{
    Object? data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) => _request<T>(
    path,
    method: 'DELETE',
    data: data,
    queryParameters: queryParameters,
    options: options,
    cancelToken: cancelToken,
    forceRetries: forceRetries,
  );

  // --------------------------
  // Core request handler
  // --------------------------
  Future<ApiResponse<T>> _request<T>(
    String path, {{
    required String method,
    Object? data,
    Map<String, dynamic>? queryParameters,
    Options? options,
    CancelToken? cancelToken,
    int? forceRetries,
  }}) async {{
    final int attempts = (forceRetries ?? maxRetries) + 1;
    int attempt = 0;

    while (true) {{
      attempt++;
      try {{
        final response = await dio.request<T>(
          path,
          data: data,
          queryParameters: queryParameters,
          options: _mergeOptions(method, options),
          cancelToken: cancelToken,
        );

        // Map success - treat 2xx as success
        final code = response.statusCode ?? 0;
        if (code >= 200 && code < 300) {{
          final msg = _messageForStatus(code, response.data);
          return ApiResponse.success(response.data, msg, statusCode: code);
        }}

        // Non-2xx -> map common codes
        final msg = _messageForStatus(code, response.data);
        // handle unauthorized hook
        if (code == 401 && onUnauthorized != null) {{
          try {{
            await onUnauthorized!();
          }} catch (_) {{
            // ignore errors from handler
          }}
        }}

        // For rate-limit (429) optionally retry
        if ((code == 429 || (code >= 500 && code < 600)) &&
            attempt < attempts) {{
          final wait = _exponentialBackoffDelay(attempt);
          await Future<void>.delayed(wait);
          continue;
        }}

        return ApiResponse.error(msg, statusCode: code);
      }} on DioException catch (err) {{
        // Handle DioError: network, timeout, cancelled, response...
        final mapped = _handleDioError(err);

        // Retry on transient errors if attempts remain
        final shouldRetry = _shouldRetryForDioError(err);
        if (shouldRetry && attempt < attempts) {{
          final wait = _exponentialBackoffDelay(attempt);
          await Future<void>.delayed(wait);
          continue;
        }}

        return ApiResponse.error(
          mapped,
          statusCode: err.response?.statusCode,
          dioError: err,
        );
    }} catch (e, st) {{
        debugPrint('[ApiServiceBase] Unexpected error: $e\n$st');
        // not a dio error: don't retry
        return ApiResponse.error('Unexpected error: ${{e.toString()}}');
    }}
    }}
    }}

  // --------------------------
  // Helpers
  // --------------------------
  Options _mergeOptions(String method, Options? options) {{
    final newOptions = options ?? Options();
    newOptions.method = method;
    // ensure headers include auth if available (sync with setAuthToken)
    if (_authToken != null) {{
      newOptions.headers ??= {{}};
      newOptions.headers!['Authorization'] = 'Bearer $_authToken';
    }}
    return newOptions;
    }}

  String _messageForStatus(int code, dynamic body) {{
    // Try to extract message from body if possible
    String messageFromBody = '';
    try {{
      if (body is Map && body['message'] != null) {{
        messageFromBody = body['message'].toString();
    }} else if (body is Map && body['error'] != null) {{
        messageFromBody = body['error']['message'].toString();
    }} else if (body is String) {{
        final regex = RegExp(r'message:\s*([^,}}]+)');
        final match = regex.firstMatch(body);

        if (match != null) {{
          messageFromBody = match.group(1)!.trim();
    }} else {{
          messageFromBody = body; // fallback
    }}
    }}
    }} catch (_) {{
      // ignore parsing errors
    }}

    switch (code) {{
      case 400:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'Bad request (400)';
      case 401:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'Unauthorized (401)';
      case 403:
        return messageFromBody.isNotEmpty ? messageFromBody : 'Forbidden (403)';
      case 404:
        return messageFromBody.isNotEmpty ? messageFromBody : 'Not found (404)';
      case 409:
        return messageFromBody.isNotEmpty ? messageFromBody : 'Conflict (409)';
      case 422:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'Validation error (422)';
      case 429:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'Too many requests (429)';
      case 500:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'Server error (500)';
      case 502:
        return 'Bad gateway (502)';
      case 503:
        return 'Service unavailable (503)';
      case 504:
        return 'Gateway timeout (504)';
      default:
        return messageFromBody.isNotEmpty
            ? messageFromBody
            : 'HTTP error ($code)';
    }}
    }}

  bool _shouldRetryForDioError(DioException err) {{
    if (err.type == DioExceptionType.connectionTimeout ||
        err.type == DioExceptionType.sendTimeout ||
        err.type == DioExceptionType.receiveTimeout) {{
      return true;
    }}
    if (err.type == DioExceptionType.unknown) {{
      // network unreachable, socket exception etc.
      return true;
    }}
    if (err.response != null && err.response?.statusCode == 429) {{
      return true;
    }}
    return false;
  }}

  String _handleDioError(DioException err) {{
    switch (err.type) {{
      case DioExceptionType.connectionTimeout:
        return 'Connection timeout';
      case DioExceptionType.sendTimeout:
        return 'Send timeout';
      case DioExceptionType.receiveTimeout:
        return 'Receive timeout';
      case DioExceptionType.badCertificate:
        return 'Bad certificate';
      case DioExceptionType.badResponse:
        // status code present - map with _messageForStatus
        final code = err.response?.statusCode ?? 0;
        return _messageForStatus(code, err.response?.data);
      case DioExceptionType.cancel:
        return 'Request cancelled';
      case DioExceptionType.connectionError:
        return 'Connection error';
      case DioExceptionType.unknown:
        // Network or unexpected error
        // if there's a response -> map properly
        if (err.response != null) {{
          final code = err.response?.statusCode ?? 0;
          return _messageForStatus(code, err.response?.data);
        }}
        return err.message ?? 'Network error';
    }}
  }}

  Duration _exponentialBackoffDelay(int attempt) {{
    // attempt starts from 1
    final ms = (pow(2, attempt) * 100).toInt(); // 200ms, 400ms, 800ms...
    // add small jitter
    final jitter = Random().nextInt(100);
    return Duration(milliseconds: ms + jitter);
  }}
}}

        "#,
            self.name
        )
    }
    fn api_constant(&self) -> String {
        format!(
            r#"
import 'package:{}/env/env.dart';

class ApiConstants {{
  static String baseEndpoint = Env.baseURL;
  static String accessToken = '';
}}
        "#,
            self.name
        )
    }
    fn cache(&self) -> String {
        r#"
import 'package:hive_flutter/hive_flutter.dart';

class CacheService {
  final _hasOnboardedDb = '_hasOnboardedDb';
  final _hasOnboardedDbKey = '_hasOnboardedDbKey';

  Future<void> saveOnboardingState(bool state) async {
    final box = await Hive.openBox(_hasOnboardedDb);
    await box.put(_hasOnboardedDbKey, state);
    await box.close();
  }

  Future<bool> getOnboardingState() async {
    final box = await Hive.openBox(_hasOnboardedDb);
    final raw = box.get(_hasOnboardedDbKey) ?? false;
    await box.close();
    return raw;
  }
}

        "#
        .to_string()
    }
    fn theme(&self) -> String {
        r#"
import 'package:flutter/material.dart';

class AppTheme {
  const AppTheme._();
  static Color primary = Colors.blue.shade500;
  static Color secondary = Colors.brown.shade200;

  static const textGrey = Color(0xFF717276);
  static const iconColor = Color(0xff7A7A7A);
  static const black = Colors.black;
  static const white = Colors.white;
  static const red = Color(0xFFB21D23);
  static TextTheme textTheme = const TextTheme(
    displayLarge: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 32,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    displayMedium: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 28,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    displaySmall: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 24,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    headlineLarge: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 22,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    headlineMedium: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 20,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    headlineSmall: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 18,
      color: textGrey,
      fontWeight: FontWeight.bold,
    ),
    titleLarge: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 16,
      fontWeight: FontWeight.bold,
      color: textGrey,
    ),
    titleMedium: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 16,
      fontWeight: FontWeight.w500,
      color: textGrey,
    ),
    titleSmall: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 14,
      fontWeight: FontWeight.w500,
      color: textGrey,
    ),
    bodyLarge: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 16,
      fontWeight: FontWeight.w400,
      color: textGrey,
    ),
    bodyMedium: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 14,
      fontWeight: FontWeight.w400,
      color: textGrey,
    ),
    bodySmall: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 12,
      fontWeight: FontWeight.w400,
      color: textGrey,
    ),
    labelLarge: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 14,
      fontWeight: FontWeight.w400,
      color: textGrey,
    ),
    labelMedium: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 12,
      fontWeight: FontWeight.w300,
      color: textGrey,
    ),
    labelSmall: TextStyle(
      fontFamily: 'Poppins',
      fontSize: 10,
      fontWeight: FontWeight.w300,
      color: textGrey,
    ),
  );

  static ThemeData light() => ThemeData(
    fontFamily: "Poppins",
    brightness: Brightness.light,
    primaryColor: primary,
    textTheme: textTheme,
    scaffoldBackgroundColor: Colors.white,
    canvasColor: secondary,
    colorScheme: ColorScheme.fromSwatch(primarySwatch: Colors.blue),
    textSelectionTheme: TextSelectionThemeData(
      cursorColor: primary,
      selectionColor: primary,
      selectionHandleColor: Colors.white,
    ),
    dialogTheme: DialogThemeData(
      backgroundColor: secondary,
      surfaceTintColor: Colors.transparent,
    ),
    appBarTheme: AppBarTheme(
      foregroundColor: primary,
      backgroundColor: secondary,
      surfaceTintColor: white,
      titleTextStyle: textTheme.titleLarge?.copyWith(color: primary),
      // systemOverlayStyle: ,
    ),
    elevatedButtonTheme: ElevatedButtonThemeData(
      style: ElevatedButton.styleFrom(
        foregroundColor: secondary,
        backgroundColor: primary,
      ),
    ),
    datePickerTheme: DatePickerThemeData(
      headerBackgroundColor: primary,
      headerForegroundColor: Colors.white,
      surfaceTintColor: Colors.transparent,
      yearOverlayColor: WidgetStateProperty.all(primary),
      dayBackgroundColor: WidgetStateProperty.resolveWith((states) {
        if (states.contains(WidgetState.selected)) {
          return primary;
        }
        return null;
      }),
      cancelButtonStyle: ButtonStyle(
        foregroundColor: WidgetStateProperty.all(black),
      ),
      confirmButtonStyle: ButtonStyle(
        foregroundColor: WidgetStateProperty.all(primary),
      ),
    ),
    bottomSheetTheme: const BottomSheetThemeData(
      backgroundColor: Colors.white,
      surfaceTintColor: Colors.white,
    ),
  );

  static ThemeData dark() {
    primary = Colors.white;
    secondary = const Color(0XFF002147);
    return ThemeData(
      fontFamily: "Poppins",
      brightness: Brightness.dark,
      primaryColor: primary,
      scaffoldBackgroundColor: secondary,
      textTheme: textTheme,
      appBarTheme: AppBarTheme(
        foregroundColor: secondary,
        backgroundColor: primary,
      ),
      elevatedButtonTheme: ElevatedButtonThemeData(
        style: ElevatedButton.styleFrom(
          foregroundColor: secondary,
          backgroundColor: primary,
        ),
      ),
    );
  }
}

        "#
        .to_string()
    }
    fn responsive_helper(&self) -> String {
        r#"
        
import 'package:flutter/material.dart';

class Breakpoints {
  static const double mobile = 600;
  static const double tablet = 1024;
  static const double desktop = 1440;
}


class ResponsiveHelper {
  static bool isMobile(BuildContext context) =>
      MediaQuery.sizeOf(context).width < Breakpoints.mobile;

  static bool isTablet(BuildContext context) =>
      MediaQuery.sizeOf(context).width >= Breakpoints.mobile &&
          MediaQuery.sizeOf(context).width < Breakpoints.tablet;

  static bool isDesktop(BuildContext context) =>
      MediaQuery.sizeOf(context).width >= Breakpoints.tablet;


  static T responsive<T>(
      BuildContext context, {
        required T mobile,
        T? tablet,
        T? desktop,
      }) {
    if (isDesktop(context) && desktop != null) return desktop;
    if (isTablet(context) && tablet != null) return tablet;
    return mobile;
  }
}
        "#
        .to_string()
    }
    fn utilities(&self) -> String {
        r#"
import 'dart:async';
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:image_picker/image_picker.dart';
import 'package:intl/intl.dart';



// pick image
pickImage(ImageSource source) async {
  final ImagePicker imagePicker = ImagePicker();
  XFile? file = await imagePicker.pickImage(source: source);

  if (file != null) {
    return await file.readAsBytes();
  }
}

// pick image as file
pickImageFile(ImageSource source) async {
  final ImagePicker imagePicker = ImagePicker();
  XFile? file = await imagePicker.pickImage(source: source);

  if (file != null) {
    return File(file.path);
  }
}

// show toast i.e. snackbar
void showSnackBar(
  String content,
  BuildContext context,
  Color color, [
  int duration = 4,
]) {
  ScaffoldMessenger.of(context).showSnackBar(
    SnackBar(
      backgroundColor: color,
      duration: Duration(seconds: duration),
      content: Text(
        content,
        style: Theme.of(
          context,
        ).textTheme.titleSmall?.copyWith(color: Colors.white),
      ),
    ),
  );
}

Future<void> showLogoutAlert(BuildContext context) async {
  final confirmed = await showDialog<bool?>(
    context: context,
    builder: (_) => AlertDialog(
      title: const Text('Do you want to logout?'),
      actions: [
        ElevatedButton(
          style: ElevatedButton.styleFrom(backgroundColor: Colors.red),
          onPressed: () {
            Navigator.of(context).pop(true);
          },
          child: const Text('Yes'),
        ),
        ElevatedButton(
          onPressed: () {
            Navigator.of(context).pop(false);
          },
          child: const Text('No'),
        ),
      ],
    ),
  );

  if (confirmed == true && context.mounted) {
    // User confirmed logout
    // logoutController(context: context);
  }
}

Future<Future> alerts(
  String title,
  String content,
  BuildContext context,
  int type, {
  VoidCallback? onTap,
}) async => showCupertinoDialog(
  context: context,
  builder: (context) {
    if (type == 0) {
      return AlertDialog(
        backgroundColor: Theme.of(context).primaryColor,
        title: Text(
          title,
          style: Theme.of(
            context,
          ).textTheme.titleLarge?.copyWith(color: Colors.white),
        ),
        content: Text(
          content,
          style: Theme.of(
            context,
          ).textTheme.titleSmall?.copyWith(color: Colors.white),
        ),
        actions: [
          TextButton(
            onPressed: () async {
              Navigator.of(context).pop();
            },
            child: Text(
              'Ok',
              style: Theme.of(
                context,
              ).textTheme.titleLarge?.copyWith(color: Colors.white),
            ),
          ),
        ],
      );
    } else if (type == 1) {
      // DELETE ACCOUNT
      return AlertDialog(
        backgroundColor: Theme.of(context).primaryColor,
        title: Text(
          title,
          style: Theme.of(
            context,
          ).textTheme.titleLarge?.copyWith(color: Colors.white),
        ),
        content: Text(
          content,
          style: Theme.of(
            context,
          ).textTheme.titleSmall?.copyWith(color: Colors.white),
        ),
        actions: [
          Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: [
              TextButton(
                onPressed: onTap,
                child: Text(
                  'Yes',
                  style: Theme.of(
                    context,
                  ).textTheme.titleMedium?.copyWith(color: Colors.redAccent),
                ),
              ),
              TextButton(
                onPressed: () async {
                  Navigator.of(context).pop();
                },
                child: Text(
                  'No',
                  style: Theme.of(context).textTheme.titleMedium?.copyWith(
                    color: Theme.of(context).scaffoldBackgroundColor,
                  ),
                ),
              ),
            ],
          ),
        ],
      );
    }
    return AlertDialog(
      backgroundColor: Theme.of(context).primaryColor,
      title: Text(
        title,
        style: Theme.of(
          context,
        ).textTheme.titleLarge?.copyWith(color: Colors.white),
      ),
      content: Text(
        content,
        style: Theme.of(
          context,
        ).textTheme.titleSmall?.copyWith(color: Colors.white),
      ),
      actions: [
        TextButton(
          style: TextButton.styleFrom(backgroundColor: Colors.white),
          onPressed: onTap,
          child: Text('Ok', style: Theme.of(context).textTheme.titleLarge),
        ),
      ],
    );
  },
);

Future<Future> redirectAlerts(
  String title,
  String content,
  BuildContext context,
  final Widget child,
) async => showCupertinoDialog(
  context: context,
  builder: (context) => CupertinoAlertDialog(
    title: Text(title, style: Theme.of(context).textTheme.titleLarge),
    content: Text(content, style: Theme.of(context).textTheme.titleSmall),
    actions: [child],
  ),
);

String addQueryParams(String basePath, Map<String, String> params) {
  if (params.isEmpty) return basePath;

  final encoded = params.entries
      .map(
        (e) => '${Uri.encodeComponent(e.key)}=${Uri.encodeComponent(e.value)}',
      )
      .join('&');

  return '$basePath?$encoded';
}

Map<String, String> getQueryParams(BuildContext context) =>
    GoRouterState.of(context).uri.queryParameters;

/// Format a number into Nigerian Naira currency format
String naira(num amount, {bool compact = false, int decimalDigits = 0}) {
  final formatter = compact
      ? NumberFormat.compactCurrency(
          locale: 'en_NG',
          symbol: '₦',
          decimalDigits: decimalDigits,
        )
      : NumberFormat.currency(
          locale: 'en_NG',
          symbol: '₦',
          decimalDigits: decimalDigits,
        );

  return formatter.format(amount);
}

        "#
        .to_string()
    }
    fn env_dart(&self) -> String {
        r#" 
import 'package:envied/envied.dart';

part 'env.g.dart';

@Envied(path: '.env')
abstract class Env {
  @EnviedField(varName: 'BASE_URL', obfuscate: true)
  static final String baseURL = _Env.baseURL;
}

        "#
        .to_string()
    }
    fn env(&self) -> String {
        String::from("BASE_URL=https://domain/api/v1")
    }
    fn responsive_builder(&self) -> String {
        r#"

import 'package:flutter/material.dart';

typedef ResponsiveWidgetBuilder =
    Widget Function(BuildContext context, BoxConstraints constraints);

class ResponsiveBuilder extends StatelessWidget {
  final ResponsiveWidgetBuilder mobile;
  final ResponsiveWidgetBuilder tablet;
  final ResponsiveWidgetBuilder desktop;

  const ResponsiveBuilder({
    super.key,
    required this.mobile,
    required this.tablet,
    required this.desktop,
  });

  static const double tabletBreakpoint = 768;
  static const double desktopBreakpoint = 1100;

  @override
  Widget build(BuildContext context) => LayoutBuilder(
      builder: (context, constraints) {
        final width = constraints.maxWidth;

        if (width >= desktopBreakpoint) {
          return desktop(context, constraints);
        }

        if (width >= tabletBreakpoint) {
          return tablet(context, constraints);
        }

        return mobile(context, constraints);
      },
    );
}

        "#
        .to_string()
    }
    fn responsive_layout(&self) -> String {
        format!(
            r#"
import 'package:flutter/material.dart';
import 'package:{name}/shared/screens/widgets/desktop_layout.dart';
import 'package:{name}/shared/screens/widgets/mobile_layout.dart';
import 'package:{name}/shared/screens/widgets/tablet_layout.dart';
import 'responsive_builder.dart';

class _ResponsiveSpacing {{
  final double spacing2;
  final double spacing4;
  final double spacing8;
  final double spacing12;
  final double spacing16;
  final double spacing24;
  final double spacing32;
  final double spacing48;
  final double spacing64;

  final double tiny;
  final double small;
  final double regular;
  final double medium;
  final double large;
  final double xLarge;

  const _ResponsiveSpacing({{
    required this.spacing2,
    required this.spacing4,
    required this.spacing8,
    required this.spacing12,
    required this.spacing16,
    required this.spacing24,
    required this.spacing32,
    required this.spacing48,
    required this.spacing64,
    required this.tiny,
    required this.small,
    required this.regular,
    required this.medium,
    required this.large,
    required this.xLarge,
  }});
}}

class ResponsiveLayout extends StatelessWidget {{
  final Widget Function(BuildContext context, BoxConstraints constraints) child;

  const ResponsiveLayout({{required this.child, super.key}});

  static late _ResponsiveSpacing _spacing;

  // Raw spacing
  static double get spacing2 => _spacing.spacing2;
  static double get spacing4 => _spacing.spacing4;
  static double get spacing8 => _spacing.spacing8;
  static double get spacing12 => _spacing.spacing12;
  static double get spacing16 => _spacing.spacing16;
  static double get spacing24 => _spacing.spacing24;
  static double get spacing32 => _spacing.spacing32;
  static double get spacing48 => _spacing.spacing48;
  static double get spacing64 => _spacing.spacing64;

  // Semantic spacing
  static double get tiny => _spacing.tiny;
  static double get small => _spacing.small;
  static double get regular => _spacing.regular;
  static double get medium => _spacing.medium;
  static double get large => _spacing.large;
  static double get xLarge => _spacing.xLarge;

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, rootConstraints) {{
      logSize(rootConstraints);
      _updateSpacing(rootConstraints);
      return ResponsiveBuilder(
        mobile: (context, _) =>
            MobileLayout(constraints: rootConstraints, child: child),

        tablet: (context, _) {{
          const maxWidth = 840.0;

          final tabletConstraints = BoxConstraints(
            maxWidth: maxWidth,
            maxHeight: rootConstraints.maxHeight,
          );

          return Center(
            child: ConstrainedBox(
              constraints: tabletConstraints,
              child: TabletLayout(constraints: tabletConstraints, child: child),
            ),
          );
        }},

        desktop: (context, _) {{
          const maxWidth = 940.0;

          final desktopConstraints = BoxConstraints(
            maxWidth: maxWidth,
            maxHeight: rootConstraints.maxHeight,
          );

          return Center(
            child: ConstrainedBox(
              constraints: desktopConstraints,
              child: DesktopLayout(
                constraints: desktopConstraints,
                child: child,
              ),
            ),
          );
        }},
      );
    }},
  );

  void logSize(BoxConstraints constraints) {{
    debugPrint(
      'maxHeight:${{constraints.maxHeight}}\nmaxWidth:${{constraints.maxWidth}}',
    );
  }}

  static void _updateSpacing(BoxConstraints c) {{
    final width = c.maxWidth;

    const mobile = 360.0;
    const tablet = 840.0;
    const desktop = 1200.0;

    double scale;

    if (width <= mobile) {{
      scale = width / mobile;
    }} else if (width <= tablet) {{
      scale = width / tablet;
    }} else {{
      scale = width / desktop;
    }}

    // Guardrails
    scale = scale.clamp(0.85, 1.25);

    double s(double v) => v * scale;

    _spacing = _ResponsiveSpacing(
      spacing2: s(2),
      spacing4: s(4),
      spacing8: s(8),
      spacing12: s(12),
      spacing16: s(16),
      spacing24: s(24),
      spacing32: s(32),
      spacing48: s(48),
      spacing64: s(64),

      // semantic aliases (derived, not duplicated logic)
      tiny: s(4),
      small: s(8),
      regular: s(16),
      medium: s(24),
      large: s(32),
      xLarge: s(48),
    );
  }}
}}

        "#,
            name = self.name
        )
    }
    fn home_index(&self) -> String {
        format!(
            r#"
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';
import 'package:{name}/core/themes/theme.dart';
import 'package:{name}/responsive/responsive_layout.dart';

const List<Widget> _screens = [
  PlaceHolderScreens(title: "Home"),
  PlaceHolderScreens(title: "Search"),
  PlaceHolderScreens(title: "Cart"),
  PlaceHolderScreens(title: "Profile"),
];

class HomeIndex extends ConsumerStatefulWidget {{
  const HomeIndex({{super.key}});

  @override
  ConsumerState<HomeIndex> createState() => _HomeIndexState();
}}

int _index = 0;

class _HomeIndexState extends ConsumerState<HomeIndex> {{
  @override
  Widget build(BuildContext context) {{
    final _fixedHeight = MediaQuery.of(context).size.height;
    return ResponsiveLayout(
      child: (context, constraints) => Scaffold(
        body: SizedBox(
          height: _fixedHeight,
          width: double.infinity,
          child: _screens[_index],
        ),
        bottomNavigationBar: BottomNavigationBar(
          selectedItemColor: AppTheme.primary,
          unselectedItemColor: Colors.black,
          type: BottomNavigationBarType.fixed,
          currentIndex: _index,
          selectedLabelStyle: const TextStyle(fontWeight: FontWeight.w700),
          unselectedLabelStyle: const TextStyle(fontWeight: FontWeight.w700),
          onTap: (index) {{
            setState(() => _index = index);
          }},
          items: [
            BottomNavigationBarItem(
              icon: Icon(MdiIcons.homeOutline),
              label: 'Home',
            ),
            BottomNavigationBarItem(
              icon: Icon(MdiIcons.magnify),
              label: 'Search',
            ),
            BottomNavigationBarItem(
              icon: Icon(MdiIcons.cartOutline),
              label: 'Cart',
            ),
            BottomNavigationBarItem(
              icon: Icon(MdiIcons.accountOutline),
              label: 'profile',
            ),
          ],
        ),
      ),
    );
  }}
}}

class PlaceHolderScreens extends StatelessWidget {{
  final String title;
  const PlaceHolderScreens({{super.key, required this.title}});

  @override
  Widget build(BuildContext context) => ResponsiveLayout(
      child: (context, constraints) => Scaffold(
        appBar: AppBar(
          title: Text(
            "Welcome to StackForge",
            style: Theme.of(
              context,
            ).textTheme.headlineMedium?.copyWith(color: Colors.black),
          ),
        ),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Center(
              child: Text(
                title,
                style: Theme.of(context).textTheme.displayLarge,
              ),
            ),
          ],
        ),
      ),
    );
}}

        "#,
            name = self.name
        )
    }
    fn main(&self) -> String {
        format!(
            r#"
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:{name}/app/app.dart';
import 'package:{name}/app/app_routes.dart';

void main() async {{
  await Hive.initFlutter();
  await loadOnboardingState();
  runApp(const ProviderScope(child: App()));
    }}

        "#,
            name = self.name
        )
    }
    fn gitignore(&self) -> String {
        r#"
# =======================================
# StackForge   .gitignore     (Secure)
# =======================================

# Flutter/Dart defaults
.dart_tool/
.packages
.pub-cache/
build/

# Generated by Flutter
flutter_*.png
.generated/
flutter_assets/

# iOS
ios/Flutter/App.framework
ios/Flutter/Flutter.framework
ios/Flutter/Flutter.podspec
ios/Flutter/Generated.xcconfig
ios/.symlinks
ios/Pods/
ios/Runner.xcworkspace/
ios/build/

# Android
android/.gradle/
android/captures/
android/app/build/
android/local.properties

# macOS
macos/Flutter/GeneratedPluginRegistrant.swift
macos/Pods/
macos/build/
macos/Runner.xcworkspace

# Windows
windows/Runner/GeneratedPluginRegistrant.*
windows/build/

# Web
web/generated_plugin_registrant.dart

# Linux
linux/runner/GeneratedPluginRegistrant.*
linux/build/

# VS Code
.vscode/

# IntelliJ / Android Studio
*.iml
.idea/
*.ipr
*.iws

# JetBrains Rider
.idea/

# Obfuscation and symbol dump directories
build/symbols/

# Secret configuration
.env
makefile
*.jks
*.p12
*.keystore
key.properties
android/key.jks
**/credentials.json
**/*.pem
**/*.key
**/*.crt

# Firebase & Google
google-services.json
GoogleService-Info.plist

# Crashlytics
.crashlytics/

# Node (if used for CI tooling)
node_modules/

# Coverage
coverage/

# Logs & temporary files
*.log
*.tmp
*.swp
*.lock

# Miscellaneous
*.class                 # Java bytecode
*.pyc                   # Python bytecode
.DS_Store               # macOS system files
.buildlog/              # Flutter build logs
migrate_working_dir/    # Dart null-safety migration temp

# Local backups
*.bak
*.orig
*.old

# Ignore exported IPA/APK/AAB builds
*.ipa
*.apk
*.aab
*.tar.gz

# Prevent accidental secret commit warnings
# Optional: track with pre-commit or secret-scanning tools

        "#
        .to_string()
    }
    fn analysis_options(&self) -> String {
        r#"
include: package:flutter_lints/flutter.yaml

linter:
  rules:
    - avoid_print
    - avoid_empty_else
    - cancel_subscriptions
    - close_sinks
    - constant_identifier_names
    - control_flow_in_finally
    - diagnostic_describe_all_properties
    - empty_statements
    - hash_and_equals
    - implementation_imports
    - library_private_types_in_public_api
    - no_duplicate_case_values
    - non_constant_identifier_names
    - prefer_const_constructors
    - prefer_const_literals_to_create_immutables
    - prefer_final_fields
    - prefer_final_locals
    - unnecessary_const
    - unnecessary_new
    - use_key_in_widget_constructors
    - always_declare_return_types
    - always_put_control_body_on_new_line
    - always_put_required_named_parameters_first
    - annotate_overrides
    - avoid_returning_null_for_future
    - avoid_types_on_closure_parameters
    - camel_case_types
    - lines_longer_than_80_chars
    - null_closures
    - only_throw_errors
    - prefer_expression_function_bodies
    - secure_pubspec_urls
    - sort_child_properties_last
    - type_annotate_public_apis
    - unnecessary_lambdas
    - prefer_single_quotes
    - use_super_parameters

analyzer:
  strong-mode:
    implicit-casts: false
    implicit-dynamic: false

  errors:
    missing_return: error
    dead_code: error
    unused_import: warning
    unused_local_variable: warning
    unused_field: warning
    unused_element: warning
    unnecessary_null_in_if_null_operators: warning
    unnecessary_nullable_for_final_variable_declarations: warning
    unnecessary_this: warning
    invalid_annotation_target: error
    todo: warning
        "#
        .to_string()
    }
    fn flutter_launcher_icons(&self) -> String {
        r##"
flutter_launcher_icons:
  android: "launcher_icon"
  ios: true
  remove_alpha_ios: true
  image_path: "assets/images/logo.png"
  adaptive_icon_background: "#8e6cef"
  adaptive_icon_foreground: "assets/images/logo.png"
        "##
        .to_string()
    }
    fn flutter_native_splash(&self) -> String {
        r##"
        
flutter_native_splash:
  ios: true
  android: true
  color: "#ffffff"
  image: assets/images/logo.png
  branding_bottom_padding: 24
  color_dark: "#ffffff"
  image_dark: assets/images/logo.png

  android_12:
    image: assets/images/logo.png
    icon_background_color: "#ffffff"
    image_dark: assets/images/logo.png
    icon_background_color_dark: "#ffffff"

  web: false
  
        "##
        .to_string()
    }
    fn make_file(&self) -> String {
        r##"
# =====================================================
# StackForge – Advanced Flutter Makefile
# =====================================================

# ------------------
# Core
# ------------------
FLUTTER      := flutter
DART         := dart
BUILD_DIR    := build
SYMBOLS      := build/symbols
COVERAGE     := coverage
APK          := $(BUILD_DIR)/app/outputs/flutter-apk/app-release.apk
AAB          := $(BUILD_DIR)/app/outputs/bundle/release/app-release.aab
IPA_DIR      := $(BUILD_DIR)/ios/ipa

.DEFAULT_GOAL := help

# ------------------
# Android Signing
# ------------------
KEYSTORE     ?= android/key.jks
KEY_ALIAS    ?= key_alias
KEY_PASS     ?= key_password
STORE_PASS   ?= store_password

# ------------------
# iOS Signing
# ------------------
IOS_TEAM     ?= YOUR_TEAM_ID
IOS_EXPORT   ?= ios/exportOptions.plist

# ------------------
# Flavors
# ------------------
FLAVOR       ?= prod
ENTRY        ?= lib/main_$(FLAVOR).dart

# ------------------
# Help
# ------------------
.PHONY: help
help:
	@echo "Core:"
	@echo "  make deps        → install deps"
	@echo "  make clean       → clean build"
	@echo "  make doctor     → flutter doctor"
	@echo ""
	@echo "Quality:"
	@echo "  make fmt         → format"
	@echo "  make analyze     → analyze"
	@echo "  make test        → tests"
	@echo "  make cov         → coverage"
	@echo "  make qa          → fmt + analyze + test"
	@echo ""
	@echo "Run:"
	@echo "  make run-a       → android"
	@echo "  make run-i       → ios"
	@echo ""
	@echo "Build:"
	@echo "  make apk         → android apk"
	@echo "  make aab         → android appbundle"
	@echo "  make ios         → ios build"
	@echo "  make ipa         → export ipa"
	@echo ""
	@echo "Advanced:"
	@echo "  make gen         → codegen"
	@echo "  make icons       → launcher icons"
	@echo "  make splash      → splash screen"
	@echo "  make version     → show version"
	@echo "  make ci          → CI pipeline"

# ------------------
# Core Workflow
# ------------------
deps:
	$(FLUTTER) pub get

upgrade:
	$(FLUTTER) pub upgrade --major-versions

clean:
	$(FLUTTER) clean
	rm -rf $(SYMBOLS) $(COVERAGE)

doctor:
	$(FLUTTER) doctor -v

# ------------------
# Quality
# ------------------
fmt:
	$(FLUTTER) format lib/ test/ --set-exit-if-changed

analyze:
	$(FLUTTER) analyze

lint:
	$(DART) analyze

test:
	$(FLUTTER) test

cov:
	$(FLUTTER) test --coverage
	genhtml $(COVERAGE)/lcov.info -o $(COVERAGE)/html

qa: fmt analyze test

# ------------------
# Codegen
# ------------------
gen:
	$(DART) run build_runner build --delete-conflicting-outputs

watch:
	$(DART) run build_runner watch

# ------------------
# Run
# ------------------
run-a:
	$(FLUTTER) run -d android --flavor $(FLAVOR) -t $(ENTRY)

run-i:
	$(FLUTTER) run -d ios --flavor $(FLAVOR) -t $(ENTRY)

# ------------------
# Android
# ------------------
apk:
	$(FLUTTER) build apk --release \
		--flavor $(FLAVOR) -t $(ENTRY) \
		--obfuscate --split-debug-info=$(SYMBOLS)

apk-debug:
	$(FLUTTER) build apk --debug --flavor $(FLAVOR)

aab:
	$(FLUTTER) build appbundle --release \
		--flavor $(FLAVOR) -t $(ENTRY)

sign-apk:
	jarsigner -keystore $(KEYSTORE) \
		-storepass $(STORE_PASS) \
		-keypass $(KEY_PASS) \
		$(APK) $(KEY_ALIAS)

install-a:
	adb install -r $(APK)

# ------------------
# iOS
# ------------------
ios:
	$(FLUTTER) build ios --release \
		--flavor $(FLAVOR) -t $(ENTRY) \
		--obfuscate --split-debug-info=$(SYMBOLS)

archive:
	xcodebuild archive \
		-workspace ios/Runner.xcworkspace \
		-scheme Runner \
		-configuration Release \
		-archivePath $(BUILD_DIR)/ios/Runner.xcarchive \
		-developmentTeam $(IOS_TEAM)

ipa:
	xcodebuild -exportArchive \
		-archivePath $(BUILD_DIR)/ios/Runner.xcarchive \
		-exportOptionsPlist $(IOS_EXPORT) \
		-exportPath $(IPA_DIR)

install-i:
	$(FLUTTER) install -d ios

# ------------------
# Branding
# ------------------
icons:
	$(DART) run flutter_launcher_icons

splash:
	$(DART) run flutter_native_splash:create

branding: icons splash

# ------------------
# Security
# ------------------
keystore:
	keytool -genkey -v \
		-keystore $(KEYSTORE) \
		-alias $(KEY_ALIAS) \
		-keyalg RSA -keysize 2048 -validity 10000

# ------------------
# Versioning
# ------------------
version:
	@grep "^version:" pubspec.yaml

bump-patch:
	$(DART) pub global run cider bump patch

bump-minor:
	$(DART) pub global run cider bump minor

bump-major:
	$(DART) pub global run cider bump major

# ------------------
# CI / Release
# ------------------
ci: deps gen qa apk ios

release: clean deps gen qa aab ipa

        "##
        .to_string()
    }
    fn read_me(&self) -> String {
        format!(
            r#"
# {}

A Flutter project scaffolded with **StackForge**.

---

## 🚀 About

This project was generated using **StackForge**, a cross-platform project scaffold builder designed to create consistent, production-ready project structures.

---

## 🛠 Getting Started

Make sure Flutter is installed and properly configured:

```bash
flutter doctor
```
Install dependencies:

```bash
flutter pub get
```

Run the app:

```bash
flutter run
```
 "#,
            self.name
        )
    }

    fn insert_packages(&self, root: &mut Mapping, key: &str, packages: Vec<(&str, &str)>) {
        let mut map = Mapping::new();

        for (name, version) in packages {
            map.insert(
                Value::String(name.to_string()),
                Value::String(version.to_string()),
            );
        }

        root.insert(Value::String(key.to_string()), Value::Mapping(map));
    }
    fn merge_packages(&self, root: &mut Mapping, key: &str, packages: Vec<(&str, &str)>) {
        let entry = root
            .entry(Value::String(key.to_string()))
            .or_insert_with(|| Value::Mapping(Mapping::new()));

        let map = entry.as_mapping_mut().unwrap();

        for (name, version) in packages {
            map.insert(
                Value::String(name.to_string()),
                Value::String(version.to_string()),
            );
        }
    }
    pub fn append_flutter_dependencies(
        &self,
        deps: Vec<(&str, &str)>,
        dev_deps: Vec<(&str, &str)>,
    ) -> Result<()> {
        let path = Path::new("pubspec.yaml");
        let content = fs::read_to_string(path)?;
        let mut yaml: Value = serde_yaml::from_str(&content)?;

        let root = yaml
            .as_mapping_mut()
            .ok_or_else(|| anyhow!("Invalid YAML root"))?;

        self.merge_packages(root, "dependencies", deps);
        self.merge_packages(root, "dev_dependencies", dev_deps);

        let clean_yaml = serde_yaml::to_string(&yaml)?;
        fs::write(path, clean_yaml)?;

        Ok(())
    }
    pub fn update_flutter_dependencies(
        &self,
        deps: Vec<(&str, &str)>,
        dev_deps: Vec<(&str, &str)>,
    ) -> Result<()> {
        let path = Path::new("pubspec.yaml");
        // Read YAML file
        let content = fs::read_to_string(path)?;
        let mut yaml: Value = serde_yaml::from_str(&content)?;

        let root = yaml
            .as_mapping_mut()
            .ok_or_else(|| anyhow::anyhow!("Invalid YAML root"))?;

        // Update dependencies
        self.insert_packages(root, "dependencies", deps);
        self.insert_packages(root, "dev_dependencies", dev_deps);

        // Write back (comments stripped automatically)
        let clean_yaml = serde_yaml::to_string(&yaml)?;
        fs::write(path, clean_yaml)?;

        Ok(())
    }
    fn desktop_layout(&self) -> String {
        r#"
import 'package:flutter/material.dart';

class DesktopLayout extends StatelessWidget {
  final Widget Function(BuildContext, BoxConstraints) child;
  final BoxConstraints constraints;

  const DesktopLayout({
    super.key,
    required this.child,
    required this.constraints,
  });

  @override
  Widget build(BuildContext context) => Padding(
      padding: const EdgeInsets.symmetric(horizontal: 32),
      child: child(context, constraints),
    );
}

    "#
        .to_string()
    }
    fn mobile_layout(&self) -> String {
        r#"
import 'package:flutter/material.dart';

class MobileLayout extends StatelessWidget {
  final Widget Function(BuildContext, BoxConstraints) child;
  final BoxConstraints constraints;

  const MobileLayout({
    super.key,
    required this.child,
    required this.constraints,
  });

  @override
  Widget build(BuildContext context) {
    return child(context, constraints);
  }
}

        "#
        .to_string()
    }
    fn tablet_layout(&self) -> String {
        r#"
import 'package:flutter/material.dart';

class TabletLayout extends StatelessWidget {
  final Widget Function(BuildContext, BoxConstraints) child;
  final BoxConstraints constraints;

  const TabletLayout({
    super.key,
    required this.child,
    required this.constraints,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 24),
      child: child(context, constraints),
    );
  }
}

        "#
        .to_string()
    }
    fn text_fields(&self) -> String {
        let import_with_name = format!(r#"import 'package:{}/core/themes/theme.dart';"#, self.name);
        let file_body = r#"
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:material_design_icons_flutter/material_design_icons_flutter.dart';

//***************************************************
//*  Input Fields                                    *
//***************************************************
/// Reusable custom input field with configurable border radius, prefix/suffix,
/// password obscuration, validators, and other common TextField properties.
class CustomInput extends StatefulWidget {
  const CustomInput({
    Key? key,
    this.controller,
    this.label,
    this.hint,
    this.initialValue,
    this.keyboardType,
    this.textInputAction,
    this.obscureText = false,
    this.prefix,
    this.suffix,
    this.borderRadius = 12.0,
    this.borderWidth = 1.2,
    this.fillColor = const Color(0xFFF3F4F6),
    this.onChanged,
    this.validator,
    this.inputFormatters,
    this.maxLines = 1,
    this.maxLength,
    this.readOnly = false,
    this.autofillHints,
    this.onTap,
    this.iconBoxConstraints,
    this.suffixText,
    this.enableBorder = true,
    this.autoFocus = true,
  }) : super(key: key);

  final TextEditingController? controller;
  final String? initialValue;
  final String? label;
  final String? hint;
  final TextInputType? keyboardType;
  final TextInputAction? textInputAction;
  final bool obscureText;
  final Widget? prefix;
  final Widget? suffix;
  final double borderRadius;
  final double borderWidth;
  final Color fillColor;
  final ValueChanged<String>? onChanged;
  final String? Function(String? value)? validator;
  final List<TextInputFormatter>? inputFormatters;
  final int maxLines;
  final int? maxLength;
  final bool readOnly;
  final Iterable<String>? autofillHints;
  final VoidCallback? onTap;
  final BoxConstraints? iconBoxConstraints;
  final bool enableBorder;
  final String? suffixText;
  final bool autoFocus;

  @override
  State<CustomInput> createState() => _CustomInputState();
}

class _CustomInputState extends State<CustomInput> {
  late bool _obscured;

  @override
  void initState() {
    super.initState();
    _obscured = widget.obscureText;
  }

  void _toggleObscure() {
    setState(() => _obscured = !_obscured);
  }

  @override
  Widget build(BuildContext context) {
    final border = OutlineInputBorder(
      borderRadius: BorderRadius.circular(widget.borderRadius),
      borderSide: widget.enableBorder
          ? BorderSide(width: widget.borderWidth, color: Colors.grey.shade400)
          : BorderSide.none,
    );

    // If user provided a suffix, use it; for password fields automatically add toggle
    Widget? suffix = widget.suffix;
    if (widget.obscureText) {
      suffix = IconButton(
        splashRadius: 18,
        icon: Icon(_obscured ? MdiIcons.eyeOffOutline : MdiIcons.eyeOutline),
        onPressed: _toggleObscure,
      );
    }

    return Column(
      spacing: 6,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.label != null)
          Text(
            widget.label!,
            style: Theme.of(context).textTheme.labelLarge?.copyWith(
              fontWeight: FontWeight.w600,
              color: Colors.black.withValues(alpha: 0.5),
            ),
          ),
        TextFormField(
          autofocus: widget.autoFocus,
          controller: widget.controller,
          initialValue: widget.initialValue,
          keyboardType: widget.keyboardType,
          textInputAction: widget.textInputAction,
          obscureText: _obscured,
          obscuringCharacter: '*',
          maxLines: widget.maxLines,
          maxLength: widget.maxLength,
          inputFormatters: widget.inputFormatters,
          readOnly: widget.readOnly,
          autofillHints: widget.autofillHints?.toList(),
          onTap: widget.onTap,
          onChanged: widget.onChanged,
          validator: widget.validator,
          decoration: InputDecoration(
            hintText: widget.hint,
            hintStyle: Theme.of(
              context,
            ).textTheme.labelLarge?.copyWith(color: AppTheme.textGrey),
            filled: widget.fillColor != null,
            fillColor: widget.fillColor,
            prefixIcon: widget.prefix,
            suffixIcon: suffix,
            suffixIconConstraints: widget.iconBoxConstraints,
            prefixIconConstraints: widget.iconBoxConstraints,
            suffixText: widget.suffixText,
            enabledBorder: border,
            focusedBorder: border.copyWith(
              borderSide: BorderSide(
                width: widget.borderWidth,
                color: Theme.of(context).primaryColor,
              ),
            ),
            errorBorder: border.copyWith(
              borderSide: BorderSide(
                width: widget.borderWidth,
                color: Colors.red,
              ),
            ),
            focusedErrorBorder: border.copyWith(
              borderSide: BorderSide(
                width: widget.borderWidth,
                color: Colors.redAccent,
              ),
            ),
            contentPadding: const EdgeInsets.symmetric(
              horizontal: 12,
              vertical: 10,
            ),
          ),
        ),
      ],
    );
  }
}

class CustomInputValidator {
  const CustomInputValidator._();

  static String? name(String? value, [String? nameType]) {
    if (value != null) {
      if (value.isEmpty) {
        return nameType == null
            ? 'Please enter your name'
            : 'Please enter your $nameType name';
      }
      if (value.length < 2) {
        return 'Name must be at least 2 characters';
      }
    }
    return null;
  }

  static String? email(String? value) {
    if (value != null) {
      if (value.isEmpty) {
        return 'Please enter your email address';
      }
      final emailRegex = RegExp(r'^[\w\.\-+%]+@[\w\.\-]+\.[a-zA-Z]{2,}$');
      if (!emailRegex.hasMatch(value)) {
        return 'Please enter a valid email address';
      }
    }
    return null;
  }

  static String? phoneNumber(String? value) {
    if (value != null) {
      if (value.isEmpty) {
        return 'Please enter your Phone number';
      }
      if (value.length < 11) {
        return 'phoneNumber must be at least 11 numbers';
      }
    }
    return null;
  }

  static String? password(String? value) {
    if (value != null) {
      if (value.isEmpty) {
        return 'Please enter a password';
      }
      if (value.length < 8) {
        return 'Must be at least 8 characters';
      }

      if (!RegExp(r'[A-Z]').hasMatch(value)) {
        return 'Must contain an uppercase letter';
      }

      if (!RegExp(r'[a-z]').hasMatch(value)) {
        return 'Must contain a lowercase letter';
      }

      if (!RegExp(r'\d').hasMatch(value)) {
        return 'Must contain a number';
      }
      if (!RegExp(r'[@$!%*?&]').hasMatch(value)) {
        return '@\$!%*?& Must be in password';
      }
    }
    return null;
  }

  static String? confirmPassword(String? value, TextEditingController ctrl) {
    if (value != null) {
      if (value.isEmpty) {
        return 'Please enter a password';
      }
      if (value != ctrl.text) {
        return 'Passwords do not match';
      }
    }
    return null;
  }
}

class OtpInput extends StatefulWidget {
  const OtpInput({
    Key? key,
    this.length = 4,
    this.boxSize = 56.0,
    this.boxSpacing = 12.0,
    this.borderRadius = 8.0,
    this.borderWidth = 1.4,
    this.borderColor = Colors.grey,
    this.fillColor = Colors.transparent,
    this.activeBorderColor,
    this.textStyle,
    this.keyboardType = TextInputType.number,
    this.autofillHints,
    this.onCompleted,
    this.enablePinAutofill = true,
  }) : super(key: key);

  final int length;
  final double boxSize;
  final double boxSpacing;
  final double borderRadius;
  final double borderWidth;
  final Color borderColor;
  final Color fillColor;
  final Color? activeBorderColor;
  final TextStyle? textStyle;
  final TextInputType keyboardType;
  final Iterable<String>? autofillHints;
  final void Function(String code)? onCompleted;
  final bool enablePinAutofill;

  @override
  State<OtpInput> createState() => _OtpInputState();
}

class _OtpInputState extends State<OtpInput> {
  late final List<TextEditingController> _controllers;
  late final List<FocusNode> _focusNodes;
  late final List<FocusNode> _rawKeyNodes; // for RawKeyboardListener
  late final ValueNotifier<int> _activeIndex; // for styling active box

  @override
  void initState() {
    super.initState();
    _controllers = List.generate(widget.length, (_) => TextEditingController());
    _focusNodes = List.generate(widget.length, (_) => FocusNode());
    _rawKeyNodes = List.generate(widget.length, (_) => FocusNode());
    _activeIndex = ValueNotifier<int>(0);

    // optionally attach autofill when available
    if (widget.enablePinAutofill && widget.autofillHints != null) {
      // Autofill only works when wrapped by AutofillGroup by the caller.
    }
  }

  @override
  void dispose() {
    for (final c in _controllers) {
      c.dispose();
    }
    for (final f in _focusNodes) {
      f.dispose();
    }
    for (final r in _rawKeyNodes) {
      r.dispose();
    }
    _activeIndex.dispose();
    super.dispose();
  }

  String get _currentCode => _controllers.map((c) => c.text).join();

  void _onChanged(String value, int index) async {
    // If user typed/pasted multiple characters in one field (paste), distribute them:
    final digitsOnly = value.replaceAll(RegExp(r'[^0-9]'), '');
    if (digitsOnly.length > 1) {
      // handle paste detected in this field
      _applyPasted(digitsOnly, index);
      return;
    }

    if (value.isNotEmpty) {
      // keep only the first char (should be one because maxLength=1)
      final ch = value.characters.first;
      _controllers[index].text = ch;
      // move cursor to end (not really needed due to single char)
      _controllers[index].selection = TextSelection.collapsed(offset: 1);
      final next = index + 1;
      if (next < widget.length) {
        _focusNodes[next].requestFocus();
        _activeIndex.value = next;
      } else {
        _focusNodes[index].unfocus();
        _activeIndex.value = index;
      }
    } else {
      // value became empty: remain focused or move back if user deleted
      // We'll not automatically move back here to avoid unexpected jumps.
      // Backspace navigation is handled in _handleKeyEvent below.
      _activeIndex.value = index;
    }

    // notify if complete
    _maybeNotifyCompleted();
  }

  void _applyPasted(String text, int startIndex) {
    final digits = text.replaceAll(RegExp(r'[^0-9]'), '');
    if (digits.isEmpty) return;

    int writeIndex = startIndex;
    for (
      int i = 0;
      i < digits.length && writeIndex < widget.length;
      i++, writeIndex++
    ) {
      _controllers[writeIndex].text = digits[i];
    }

    // move focus to next empty or to last
    final nextEmpty = _controllers.indexWhere((c) => c.text.isEmpty);
    if (nextEmpty >= 0) {
      _focusNodes[nextEmpty].requestFocus();
      _activeIndex.value = nextEmpty;
    } else {
      // all filled
      _focusNodes.last.unfocus();
      _activeIndex.value = widget.length - 1;
    }

    _maybeNotifyCompleted();
    setState(() {}); // update visuals
  }

  Future<void> pasteFromClipboard([int startIndex = 0]) async {
    final data = await Clipboard.getData('text/plain');
    if (data == null || data.text == null) return;
    final digits = data.text!.replaceAll(RegExp(r'[^0-9]'), '');
    if (digits.isEmpty) return;
    _applyPasted(digits, startIndex);
  }

  void _maybeNotifyCompleted() {
    final code = _currentCode;
    if (code.length == widget.length && !code.contains('')) {
      if (!code.contains('')) {
        // nothing
      }
      // ensure all digits present
      if (code.length == widget.length &&
          !code.contains(RegExp(r'\s')) &&
          !code.contains('')) {
        widget.onCompleted?.call(code);
      }
    }
    // better check explicitly:
    final allFilled = _controllers.every((c) => c.text.trim().isNotEmpty);
    if (allFilled) widget.onCompleted?.call(_currentCode);
  }

  // handle keyboard events to catch backspace and move focus backward
  KeyEventResult _handleKeyEvent(FocusNode node, RawKeyEvent event, int index) {
    if (event is RawKeyDownEvent) {
      // Backspace
      if (event.logicalKey == LogicalKeyboardKey.backspace) {
        final txt = _controllers[index].text;
        if (txt.isEmpty) {
          // move to previous if exists and clear it
          final prev = index - 1;
          if (prev >= 0) {
            _controllers[prev].clear();
            _focusNodes[prev].requestFocus();
            _activeIndex.value = prev;
          }
        } else {
          // there is content, clear it (the field itself)
          _controllers[index].clear();
        }
        setState(() {});
        return KeyEventResult.handled;
      }

      // Ctrl+V or Command+V paste (desktop)
      final isPaste =
          (event.isControlPressed &&
              event.logicalKey == LogicalKeyboardKey.keyV) ||
          (event.isMetaPressed && event.logicalKey == LogicalKeyboardKey.keyV);
      if (isPaste) {
        // programmatic clipboard paste; start distributing from this index
        pasteFromClipboard(index);
        return KeyEventResult.handled;
      }
    }
    return KeyEventResult.ignored;
  }

  Widget _buildBox(int index) {
    final controller = _controllers[index];
    final focusNode = _focusNodes[index];
    final rawNode = _rawKeyNodes[index];

    return RawKeyboardListener(
      focusNode: rawNode,
      onKey: (event) => _handleKeyEvent(rawNode, event, index),
      child: ValueListenableBuilder<int>(
        valueListenable: _activeIndex,
        builder: (_, active, __) {
          final bool isActive = active == index;
          return GestureDetector(
            onTap: () {
              _focusNodes[index].requestFocus();
              _activeIndex.value = index;
            },
            onLongPress: () async {
              // show paste option via manual pasteFromClipboard
              await pasteFromClipboard(index);
            },
            child: SizedBox(
              width: widget.boxSize,
              height: widget.boxSize,
              child: TextFormField(
                controller: controller,

                focusNode: focusNode,
                keyboardType: widget.keyboardType,
                textAlign: TextAlign.center,
                maxLength: 1,
                style:
                    widget.textStyle ??
                    const TextStyle(fontSize: 20, fontWeight: FontWeight.w600),
                cursorWidth: 1.4,
                showCursor: true,
                decoration: InputDecoration(
                  counterText: '',
                  filled: true,
                  fillColor: widget.fillColor,
                  enabledBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(widget.borderRadius),
                    borderSide: BorderSide(
                      width: widget.borderWidth,
                      color: widget.borderColor,
                    ),
                  ),
                  focusedBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(widget.borderRadius),
                    borderSide: BorderSide(
                      width: widget.borderWidth,
                      color:
                          widget.activeBorderColor ??
                          Theme.of(context).primaryColor,
                    ),
                  ),
                ),
                inputFormatters: [FilteringTextInputFormatter.digitsOnly],
                autofillHints: widget.autofillHints?.toList(),
                onChanged: (v) => _onChanged(v, index),
              ),
            ),
          );
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) => Row(
    mainAxisSize: MainAxisSize.min,
    children: List.generate(
      widget.length,
      (i) => Padding(
        padding: EdgeInsets.only(
          right: i == widget.length - 1 ? 0 : widget.boxSpacing,
        ),
        child: _buildBox(i),
      ),
    ),
  );
}

class CopyTxtField extends StatefulWidget {
  const CopyTxtField({
    super.key,
    required this.context,
    required this.controller,
  });

  final BuildContext context;
  final TextEditingController controller;

  @override
  State<CopyTxtField> createState() => _CopyTxtFieldState();
}

class _CopyTxtFieldState extends State<CopyTxtField> {
  @override
  Widget build(BuildContext context) => Container(
    margin: const EdgeInsets.symmetric(vertical: 10.0),
    // color: AppTheme.inputBg,
    width: MediaQuery.of(context).size.width,
    child: TextFormField(
      controller: widget.controller,

      readOnly: true,
      textAlignVertical: TextAlignVertical.center,
      style: Theme.of(context).textTheme.bodyMedium,
      cursorColor: Theme.of(context).primaryColor,
      // enabled: false,
      decoration: InputDecoration(
        suffixIcon: InkWell(
          onTap: () {
            Clipboard.setData(ClipboardData(text: widget.controller.text)).then(
              (value) {
                //only if ->
                final snackBar = SnackBar(
                  content: const Text('Copied to Clipboard'),
                  backgroundColor: AppTheme.primary,
                  action: SnackBarAction(
                    label: 'Undo',
                    onPressed: () {
                      Clipboard.setData(const ClipboardData(text: ''));
                    },
                  ),
                );
                ScaffoldMessenger.of(context).showSnackBar(snackBar);
              },
            );
          },
          child: Icon(
            Icons.copy,
            // color: QuickBillThemes.quickBillOrange,
          ),
        ),
        labelStyle: const TextStyle(
          fontSize: 14,
          color: Colors.black,
          fontWeight: FontWeight.w500,
        ),
        alignLabelWithHint: true,
        hintStyle: const TextStyle(
          fontSize: 14,
          color: Colors.black,
          fontWeight: FontWeight.w500,
        ),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide:
              BorderSide.none, //const BorderSide(color: Color(0xFFD8DADC)),
        ),
        /*enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(10),
          borderSide: const BorderSide(color: Color(0xFFD8DADC)),
        ),
        focusedBorder: OutlineInputBorder(
          gapPadding: 0.0,
          borderRadius: BorderRadius.circular(10),
          borderSide: const BorderSide(color: Color(0xFFD8DADC)),
        ),*/
        isDense: true,
        // contentPadding: const EdgeInsets.all(10),
        fillColor: AppTheme.primary,
        focusColor: AppTheme.secondary,
        filled: true,
      ),
    ),
  );
}

typedef DropdownItemBuilder<T> = Widget Function(BuildContext context, T item);
typedef DropdownValueToString<T> = String Function(T item);

class CustomContainerDropdown<T> extends StatefulWidget {
  const CustomContainerDropdown({
    Key? key,
    required this.items,
    required this.itemBuilder,
    required this.onChanged,
    this.value,
    this.hint,
    this.label,
    this.prefixIcon,
    this.borderRadius = 12,
    this.elevation = 0,
    this.dropdownMaxHeight = 300,
    this.dropdownWidth,
    this.horizontalMargin = 16,
    this.searchable = false,
    this.itemToString,
  }) : super(key: key);

  final List<T> items;
  final DropdownItemBuilder<T> itemBuilder;
  final ValueChanged<T?> onChanged;
  final T? value;
  final String? hint;
  final String? label;
  final Widget? prefixIcon;
  final double borderRadius;
  final double elevation;
  final double dropdownMaxHeight;
  final double?
  dropdownWidth; // if null, menu width is computed from available space and horizontalMargin
  final double horizontalMargin; // padding from screen edges
  final bool searchable;
  final DropdownValueToString<T>?
  itemToString; // fallback string for search/display

  @override
  State<CustomContainerDropdown<T>> createState() =>
      _CustomContainerDropdownState<T>();
}

class _CustomContainerDropdownState<T>
    extends State<CustomContainerDropdown<T>> {
  final LayerLink _layerLink = LayerLink();
  OverlayEntry? _overlayEntry;
  bool _isOpen = false;
  late List<T> _filteredItems;
  final TextEditingController _searchController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _filteredItems = List<T>.from(widget.items);
  }

  @override
  void didUpdateWidget(covariant CustomContainerDropdown<T> oldWidget) {
    super.didUpdateWidget(oldWidget);
    // refresh items when parent updates
    _filteredItems = List<T>.from(widget.items);
  }

  @override
  void dispose() {
    _removeOverlay();
    _searchController.dispose();
    super.dispose();
  }

  void _showOverlay() {
    if (_isOpen) return;
    _overlayEntry = _buildOverlayEntry();
    Overlay.of(context, rootOverlay: true)?.insert(_overlayEntry!);
    setState(() => _isOpen = true);
  }

  void _removeOverlay() {
    _overlayEntry?.remove();
    _overlayEntry = null;
    _searchController.clear();
    _filteredItems = List<T>.from(widget.items);
    setState(() => _isOpen = false);
  }

  void _toggleOverlay() {
    if (_isOpen) {
      _removeOverlay();
    } else {
      _showOverlay();
    }
  }

  OverlayEntry _buildOverlayEntry() {
    final renderBox = context.findRenderObject() as RenderBox;
    final size = renderBox.size;
    final anchorOffset = renderBox.localToGlobal(Offset.zero);

    final mq = MediaQuery.of(context);
    final screenWidth = mq.size.width;
    final horizontalMargin = widget.horizontalMargin.clamp(
      0.0,
      screenWidth / 4,
    );

    // Compute available width and menu width (so menu won't touch edges)
    final availableWidth = screenWidth - horizontalMargin * 2;
    final menuWidth = (widget.dropdownWidth != null)
        ? widget.dropdownWidth!.clamp(0.0, availableWidth)
        : availableWidth;

    // Compute menu left position so it aligns with the field but doesn't exceed margins
    double left = anchorOffset.dx;
    // If the field left is too close to the right edge, shift left
    if (left + menuWidth > screenWidth - horizontalMargin) {
      left = (screenWidth - horizontalMargin) - menuWidth;
    }
    // Ensure left is not less than horizontalMargin
    left = left < horizontalMargin ? horizontalMargin : left;

    // top position right below the field
    final top = anchorOffset.dy + size.height + 8; // small gap
    final bottomSpace = mq.size.height - top;

    // If not enough room at bottom, show above the field
    final bool showAbove =
        bottomSpace < 120 && anchorOffset.dy > widget.dropdownMaxHeight;
    final menuTop = showAbove
        ? anchorOffset.dy - widget.dropdownMaxHeight - 8
        : top;

    return OverlayEntry(
      builder: (context) => Stack(
        children: [
          // capture taps outside to dismiss
          Positioned.fill(
            child: GestureDetector(
              behavior: HitTestBehavior.translucent,
              onTap: _removeOverlay,
              child: const SizedBox.expand(),
            ),
          ),

          // dropdown menu
          Positioned(
            left: left,
            width: menuWidth,
            top: menuTop,
            child: CompositedTransformFollower(
              link: _layerLink,
              showWhenUnlinked: false,
              child: Material(
                elevation: widget.elevation,
                color: Colors.transparent,
                child: Container(
                  constraints: BoxConstraints(
                    maxHeight: widget.dropdownMaxHeight,
                    minWidth: 120,
                  ),
                  decoration: BoxDecoration(
                    color: Theme.of(context).cardColor,
                    borderRadius: BorderRadius.circular(widget.borderRadius),
                    boxShadow: const [
                      BoxShadow(
                        color: Colors.black26,
                        blurRadius: 8,
                        offset: Offset(0, 4),
                      ),
                    ],
                  ),
                  child: ClipRRect(
                    borderRadius: BorderRadius.circular(widget.borderRadius),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        if (widget.searchable) _buildSearchField(),
                        Expanded(
                          child: _filteredItems.isEmpty
                              ? Center(
                                  child: Padding(
                                    padding: const EdgeInsets.all(16.0),
                                    child: Text(
                                      'No results',
                                      style: Theme.of(context)
                                          .textTheme
                                          .bodyMedium
                                          ?.copyWith(color: Colors.grey),
                                    ),
                                  ),
                                )
                              : Scrollbar(
                                  child: ListView.separated(
                                    padding: EdgeInsets.zero,
                                    shrinkWrap: true,
                                    itemCount: _filteredItems.length,
                                    separatorBuilder: (_, __) =>
                                        const Divider(height: 1),
                                    itemBuilder: (ctx, index) {
                                      final item = _filteredItems[index];
                                      return InkWell(
                                        onTap: () {
                                          widget.onChanged(item);
                                          _removeOverlay();
                                        },
                                        child: widget.itemBuilder(ctx, item),
                                      );
                                    },
                                  ),
                                ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSearchField() => Padding(
    padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 8.0),
    child: TextField(
      controller: _searchController,
      decoration: InputDecoration(
        isDense: true,
        hintText: 'Search...',
        prefixIcon: const Icon(Icons.search, size: 20),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(widget.borderRadius - 4),
        ),
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 12,
          vertical: 10,
        ),
      ),
      onChanged: (q) {
        final valueToString =
            widget.itemToString ??
            (T item) => item?.toString() ?? ''; // fallback
        setState(() {
          if (q.isEmpty) {
            _filteredItems = List<T>.from(widget.items);
          } else {
            final lower = q.toLowerCase();
            _filteredItems = widget.items
                .where((it) => valueToString(it).toLowerCase().contains(lower))
                .toList();
          }
        });
      },
    ),
  );

  @override
  Widget build(BuildContext context) {
    final displayText = widget.value != null
        ? (widget.itemToString ?? (T i) => i.toString())(widget.value as T)
        : (widget.hint ?? '');

    return Column(
      spacing: 7,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (widget.label != null)
          Text(
            widget.label!,
            style: Theme.of(context).textTheme.labelLarge?.copyWith(
              fontWeight: FontWeight.w600,
              color: Colors.black.withValues(alpha: 0.5),
            ),
          ),
        CompositedTransformTarget(
          link: _layerLink,
          child: GestureDetector(
            onTap: _toggleOverlay,
            child: Container(
              decoration: BoxDecoration(
                color: const Color(0xFFF3F4F6),
                borderRadius: BorderRadius.circular(widget.borderRadius),
                border: Border.all(color: Colors.grey.shade400),
              ),
              padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
              child: Row(
                children: [
                  if (widget.prefixIcon != null) ...[
                    Padding(
                      padding: const EdgeInsets.only(right: 8.0),
                      child: widget.prefixIcon!,
                    ),
                  ],
                  Expanded(
                    child: Text(
                      displayText,
                      style: widget.value != null
                          ? Theme.of(context).textTheme.bodyMedium
                          : TextStyle(
                              fontSize: 14,
                              color: Colors.black.withValues(alpha: 0.3),
                              fontWeight: FontWeight.w500,
                            ),

                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  AnimatedRotation(
                    turns: _isOpen ? 0.5 : 0.0,
                    duration: const Duration(milliseconds: 200),
                    child: const Icon(Icons.arrow_drop_down),
                  ),
                ],
              ),
            ),
          ),
        ),
      ],
    );
  }
}

Widget smallEntryField(
  BuildContext context,
  String title,
  TextEditingController controller, {
  bool isPassword = false,
  TextInputType keyboardType = TextInputType.phone,
  FocusNode? focusNode,
  FocusNode? nextFocus,
  Function? check,
  Function? paste,
}) => Container(
  margin: const EdgeInsets.symmetric(vertical: 10),
  width: MediaQuery.of(context).size.width * .128,
  decoration: BoxDecoration(
    color: isPassword ? Colors.grey.shade200 : const Color(0xFFF3F4F6),
    borderRadius: BorderRadius.circular(10.0),
  ),
  child: TextField(
    controller: controller,
    keyboardType: keyboardType,
    inputFormatters: [LengthLimitingTextInputFormatter(1)],
    onChanged: (value) {
      if (check != null) {
        check(value);
      }
      if (value.isNotEmpty && nextFocus != null) {
        FocusScope.of(context).requestFocus(nextFocus);
      } else if (value.isNotEmpty && nextFocus == null) {
        focusNode?.unfocus();
      } else {
        FocusScope.of(context).previousFocus();
      }
    },
    textInputAction: TextInputAction.next,
    focusNode: focusNode,
    obscureText: isPassword,
    obscuringCharacter: '●',
    style: isPassword
        ? TextStyle(fontSize: 22, color: Theme.of(context).primaryColor)
        : TextStyle(fontSize: 18),
    textAlign: TextAlign.center,
    cursorColor: Theme.of(context).primaryColor,
    decoration: InputDecoration(
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: BorderSide.none,
      ),
      focusedBorder: OutlineInputBorder(
        borderSide: BorderSide.none,
        borderRadius: BorderRadius.circular(10),
      ),
      fillColor: Colors.transparent,
      filled: true,
      isDense: true,
    ),
    contextMenuBuilder: (context, editableTextState) {
      bool isIOS = Theme.of(context).platform == TargetPlatform.iOS;
      return AdaptiveTextSelectionToolbar(
        anchors: editableTextState.contextMenuAnchors,
        children: editableTextState.contextMenuButtonItems
            .map(
              (buttonItem) => CupertinoButton(
                borderRadius: null,
                // color: const Color(0xffaaaa00),
                disabledColor: const Color(0xffaaaaff),
                onPressed: () {
                  paste!();
                },
                padding: const EdgeInsets.all(10.0),
                pressedOpacity: 0.7,
                child: SizedBox(
                  width: 200.0,
                  child: Text(
                    CupertinoTextSelectionToolbarButton.getButtonLabel(
                      context,
                      buttonItem,
                    ),
                  ),
                ),
              ),
            )
            .toList(),
      );
    },
  ),
);


        "#
        .to_string();
        format!(r#"{}{}"#, import_with_name, file_body)
    }
    //
    //
    pub fn files(&self) -> Vec<(String, String)> {
        vec![
            ("lib/app/app_routes.dart".to_string(), self.app_routes()),
            ("lib/app/app.dart".to_string(), self.app()),
            ("lib/app/navigation.dart".to_string(), self.navigation()),
            (
                "lib/core/services/api_base_class.dart".to_string(),
                self.api_base_class(),
            ),
            (
                "lib/core/services/api_constant.dart".to_string(),
                self.api_constant(),
            ),
            ("lib/core/services/cache.dart".to_string(), self.cache()),
            ("lib/core/themes/theme.dart".to_string(), self.theme()),
            (
                "lib/core/utils/responsive_helper.dart".to_string(),
                self.responsive_helper(),
            ),
            (
                "lib/core/utils/utilities.dart".to_string(),
                self.utilities(),
            ),
            ("lib/env/env.dart".to_string(), self.env_dart()),
            (
                "lib/responsive/responsive_builder.dart".to_string(),
                self.responsive_builder(),
            ),
            (
                "lib/responsive/responsive_layout.dart".to_string(),
                self.responsive_layout(),
            ),
            (
                "lib/shared/screens/widgets/desktop_layout.dart".to_string(),
                self.desktop_layout(),
            ),
            (
                "lib/shared/screens/widgets/mobile_layout.dart".to_string(),
                self.mobile_layout(),
            ),
            (
                "lib/shared/screens/widgets/tablet_layout.dart".to_string(),
                self.tablet_layout(),
            ),
            (
                "lib/shared/screens/widgets/text_fields.dart".to_string(),
                self.text_fields(),
            ),
            (
                "lib/shared/screens/home/index.dart".to_string(),
                self.home_index(),
            ),
            ("lib/main.dart".to_string(), self.main()),
            (".env".to_string(), self.env()),
            (".gitignore".to_string(), self.gitignore()),
            ("analysis_options.yaml".to_string(), self.analysis_options()),
            (
                "flutter_launcher_icons.yaml".to_string(),
                self.flutter_launcher_icons(),
            ),
            (
                "flutter_native_splash.yaml".to_string(),
                self.flutter_native_splash(),
            ),
            ("Makefile".to_string(), self.make_file()),
            ("README.md".to_string(), self.read_me()),
            ("test/widget_test.dart".to_string(), "".to_string()),
        ]
    }
}
